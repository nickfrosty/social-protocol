import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import type { Social } from "../target/types/social";
import { assert, expect } from "chai";

// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.Social as Program<Social>;
const payer = provider.wallet;

// init the random seed bytes to use
const random_seed_profile = anchor.web3.Keypair.generate().publicKey.toBytes();
const random_seed_profile2 = anchor.web3.Keypair.generate().publicKey.toBytes();
const random_seed_post = anchor.web3.Keypair.generate().publicKey.toBytes();
const random_seed_reply = anchor.web3.Keypair.generate().publicKey.toBytes();

// derive the pda address based on the random
const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("profile"), random_seed_profile],
  program.programId,
);

const [profilePda2] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("profile"), random_seed_profile2],
  program.programId,
);

// derive the pda address based on the random
const [postPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("post"), random_seed_post],
  program.programId,
);
const [replyPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("post"), random_seed_reply],
  program.programId,
);

describe("profile", () => {
  //
  const profileData: anchor.IdlAccounts<Social>["profile"] = {
    bump: 0, // this is ignored
    randomSeed: random_seed_profile as unknown as number[],
    authority: payer.publicKey,
    name: "name_default",
    username: "username_default",
    imageUri: "imageUri_default",
    metadataUri: "metadataUri_default",
  };

  //
  it("create profile", async () => {
    console.log("\t", "profile address:", profilePda.toBase58());

    const [nameServicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("name_service", "utf8"),
        Buffer.from("profile", "utf8"),
        Buffer.from(profileData.username, "utf8"),
      ],
      program.programId,
    );

    await program.methods
      .createProfile(profileData)
      .accounts({
        // payer: payer.publicKey,
        authority: payer.publicKey,
        profile: profilePda,
        nameService: nameServicePda,
      })
      .rpc();

    // get the profile record from the chain
    const profile = await program.account.profile.fetch(profilePda);

    // perform the assertions
    assert(profile.username === profileData.username, "Expected 'username' to match");
    assert(profile.name === profileData.name, "Expected 'name' to match");
    assert(profile.metadataUri === profileData.metadataUri, "Expected 'metadataUri' to match");
    assert(profile.imageUri === profileData.imageUri, "Expected 'imageUri' to match");
  });

  //
  it("profile name service created", async () => {
    // derive the profile's name service account from the profile's username
    const [nameServicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("name_service", "utf8"),
        Buffer.from("profile", "utf8"),
        Buffer.from(profileData.username, "utf8"),
      ],
      program.programId,
    );

    console.log("\t", "name service address:", nameServicePda.toBase58());

    const name_service = await program.account.nameService.fetch(nameServicePda);

    assert(
      name_service.address.toBase58() === profilePda.toBase58(),
      "Expected 'address' to be the profile pda",
    );

    // get the profile from the name service
    const profile = await program.account.profile.fetch(name_service.address);
    assert(
      name_service.authority.toBase58() === profile.authority.toBase58(),
      "Expected 'authority' to be the profile's owner",
    );
  });

  //
  it("update profile", async () => {
    //
    const newProfileData: anchor.IdlAccounts<Social>["profile"] = {
      bump: null,
      randomSeed: random_seed_profile as unknown as number[],
      authority: payer.publicKey,
      username: "does not change",
      name: "new name",
      imageUri: "new imageUri",
      metadataUri: "new metadataUri",
    };

    await program.methods
      .updateProfile(newProfileData)
      .accounts({
        // payer: payer.publicKey,
        profile: profilePda,
      })
      .rpc();

    // get the updated profile record from the chain
    const profile = await program.account.profile.fetch(profilePda);

    // perform the assertions
    assert(profile.name === newProfileData.name, "Expected 'name' to update");
    assert(profile.metadataUri === newProfileData.metadataUri, "Expected 'metadataUri' to update");
    assert(profile.imageUri === newProfileData.imageUri, "Expected 'imageUri' to update");

    // todo
  });

  //
  it("change username: incorrect authority", async () => {
    // define the new username
    const new_username = "brand_new_not_taken";

    // derive the profile's name service account from the profile's username
    const [oldNameServicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("name_service", "utf8"),
        Buffer.from("profile", "utf8"),
        Buffer.from(profileData.username, "utf8"),
      ],
      program.programId,
    );

    const [newNameServicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("name_service", "utf8"),
        Buffer.from("profile", "utf8"),
        Buffer.from(new_username, "utf8"),
      ],
      program.programId,
    );

    const wrongAuthority = anchor.web3.Keypair.generate();

    // ensure the old name service account was closed
    try {
      await program.methods
        .changeUsername(random_seed_profile as unknown as number[], new_username)
        .accounts({
          // note: when not provided, Anchor should auto-magically set this to the fee payer
          authority: wrongAuthority.publicKey,
          profile: profilePda,
          oldNameService: oldNameServicePda,
          newNameService: newNameServicePda,
        })
        .signers([wrongAuthority])
        .rpc();
    } catch (err) {
      expect(err).to.be.an("Error");
      expect(err.message).to.contain(`Unauthorized`);
    }
  });

  //
  it("change username: correct authority", async () => {
    // set a new usernames
    const new_username = "brand_new_not_taken";

    // derive the profile's name service account from the profile's username
    const [oldNameServicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("name_service", "utf8"),
        Buffer.from("profile", "utf8"),
        Buffer.from(profileData.username, "utf8"),
      ],
      program.programId,
    );

    const [newNameServicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("name_service", "utf8"),
        Buffer.from("profile", "utf8"),
        Buffer.from(new_username, "utf8"),
      ],
      program.programId,
    );
    console.log("\t", "new name service address:", newNameServicePda.toBase58());

    await program.methods
      .changeUsername(random_seed_profile as unknown as number[], new_username)
      .accounts({
        // note: when not provided, Anchor should auto-magically set this to the fee payer
        // authority: payer.publicKey,
        profile: profilePda,
        oldNameService: oldNameServicePda,
        newNameService: newNameServicePda,
      })
      .rpc();

    // get the updated profile record from the chain
    const new_name_service = await program.account.nameService.fetch(newNameServicePda);

    // ensure the new name service has the correct data
    assert(
      new_name_service.address.toBase58() === profilePda.toBase58(),
      "Expected 'address' to be the profile pda",
    );

    // ensure the old name service account was closed
    try {
      await program.account.nameService.fetch(oldNameServicePda);
    } catch (err) {
      expect(err).to.be.an("Error");
      expect(err.message).to.contain(`Account does not exist or has no data`);
    }
  });
});

describe("post", () => {
  //
  it("create post", async () => {
    console.log("\t", "post address:", postPda.toBase58());

    const metadataUri = "metadataUri_default";

    await program.methods
      .createPost(random_seed_post as unknown as number[], metadataUri)
      .accounts({
        author: profilePda,
        post: postPda,
      })
      .rpc();

    // get the post record from the chain
    const post = await program.account.post.fetch(postPda);

    // perform the assertions
    assert(
      post.author.toBase58() === profilePda.toBase58(),
      "Expected 'author' to be the 'profilePda'",
    );
    assert(post.metadataUri === metadataUri, "Expected 'metadataUri' to match");
  });

  //
  it("update post: incorrect authority", async () => {
    //
    const metadataUri = "failing metadataUri";

    const wrongAuthority = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .updatePost(random_seed_post as unknown as number[], metadataUri)
        .accounts({
          // note: when not provided, Anchor should auto-magically set this to the fee payer
          authority: wrongAuthority.publicKey,
          author: profilePda,
          post: postPda,
        })
        .signers([wrongAuthority])
        .rpc();
    } catch (err) {
      expect(err).to.be.an("Error");
      expect(err.message).to.contain("Unauthorized");
      // todo: add more specific error message trigger
    }

    // get the updated profile record from the chain
    const post = await program.account.post.fetch(postPda);

    // perform the assertions
    assert(post.metadataUri !== metadataUri, "Expected 'metadataUri' to NOT update");

    // todo
  });

  //
  it("update post: correct authority", async () => {
    //
    const metadataUri = "winning metadataUri";

    await program.methods
      .updatePost(random_seed_post as unknown as number[], metadataUri)
      .accounts({
        // note: when not provided, Anchor should auto-magically set this to the fee payer
        // authority: payer.publicKey,
        author: profilePda,
        post: postPda,
      })
      .rpc();

    // get the updated profile record from the chain
    const post = await program.account.post.fetch(postPda);

    // perform the assertions
    assert(post.metadataUri === metadataUri, "Expected 'metadataUri' to update");

    // todo
  });

  //
  it("create reply", async () => {
    console.log("\t", "reply address:", replyPda.toBase58());

    const metadataUri = "reply_metadataUri";

    await program.methods
      .createReply(random_seed_reply as unknown as number[], metadataUri)
      .accounts({
        author: profilePda,
        parentPost: postPda,
        reply: replyPda,
      })
      .rpc();

    // get the parent post and reply record from the chain
    const post = await program.account.post.fetch(postPda);
    const reply = await program.account.post.fetch(replyPda);

    // perform the assertions
    assert(
      reply.author.toBase58() === profilePda.toBase58(),
      "Expected 'author' to be the 'profilePda'",
    );
    assert(reply.metadataUri === metadataUri, "Expected 'metadataUri' to match");
    assert(
      reply.parentPost.toBase58() === postPda.toBase58(),
      "Expected parent post to be 'postPda'",
    );
    assert(post.replyCount === 1, "Expected 'reply_count' to increment");
  });

  //
  it("update reply: correct authority", async () => {
    //
    const metadataUri = "new reply metadataUri";

    await program.methods
      .updatePost(random_seed_reply as unknown as number[], metadataUri)
      .accounts({
        // note: when not provided, Anchor should auto-magically set this to the fee payer
        // authority: payer.publicKey,
        author: profilePda,
        post: replyPda,
      })
      .rpc();

    // get the updated profile record from the chain
    const reply = await program.account.post.fetch(replyPda);

    // perform the assertions
    assert(reply.metadataUri === metadataUri, "Expected 'metadataUri' to update");

    // todo
  });
});
