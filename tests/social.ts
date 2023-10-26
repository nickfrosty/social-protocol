import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import type { Social } from "../target/types/social";

import chai, { expect, assert } from "chai";
import chaiAsPromised from "chai-as-promised";
import {
  deriveNameServiceAddress,
  derivePostAddress,
  derivePostGroupAddress,
  deriveProfileAddress,
} from "../client/accounts";

chai.use(chaiAsPromised);

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
const random_seed_postGroup = anchor.web3.Keypair.generate().publicKey.toBytes();

// derive the pda address based on the random
const [profilePda] = deriveProfileAddress(random_seed_profile);
const [profilePda2] = deriveProfileAddress(random_seed_profile2);

// derive the pda address based on the random
const [postPda] = derivePostAddress(random_seed_post);
const [replyPda] = derivePostAddress(random_seed_reply);

//
const [postGroupPda] = derivePostGroupAddress(random_seed_postGroup);

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

    const [nameServicePda] = deriveNameServiceAddress("profile", profileData.username);

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

    // note: the name service checks for profile creation are next

    // perform the assertions on the profile
    assert(
      profile.authority.toBase58() === payer.publicKey.toBase58(),
      "Expected the payer to be profile's authority",
    );
    assert(profile.username === profileData.username, "Expected 'username' to match");
    assert(profile.name === profileData.name, "Expected 'name' to match");
    assert(profile.metadataUri === profileData.metadataUri, "Expected 'metadataUri' to match");
    assert(profile.imageUri === profileData.imageUri, "Expected 'imageUri' to match");
  });

  //
  it("create profile name service", async () => {
    // derive the profile's name service account from the profile's username
    const [nameServicePda] = deriveNameServiceAddress("profile", profileData.username);

    console.log("\t", "name service address:", nameServicePda.toBase58());

    // get the name service and profile from the blockchain
    const name_service = await program.account.nameService.fetch(nameServicePda);
    const updatedProfile = await program.account.profile.fetch(name_service.address);

    // ensure the name service record points to the correct record address
    const [nameServiceTestPda] = deriveNameServiceAddress("profile", updatedProfile.username);
    assert(
      nameServicePda.toBase58() === nameServiceTestPda.toBase58(),
      "Expected the 'nameServicePda' to be derived from the 'updatedProfile.username'",
    );

    assert(
      updatedProfile.authority.toBase58() === payer.publicKey.toBase58(),
      "Expected 'payer' to be the profile authority",
    );
    assert(
      name_service.address.toBase58() === profilePda.toBase58(),
      "Expected 'address' to be the profile pda",
    );
    assert(
      name_service.authority.toBase58() === profilePda.toBase58(),
      "Expected 'authority' to to be the profile pda",
    );
  });

  //
  it("update profile: incorrect authority", async () => {
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

    const wrongAuthority = anchor.web3.Keypair.generate();

    await expect(
      program.methods
        .updateProfile(newProfileData)
        .accounts({
          authority: wrongAuthority.publicKey,
          profile: profilePda,
        })
        .signers([wrongAuthority])
        .rpc(),
    ).to.eventually.be.rejectedWith(
      "AnchorError caused by account: profile. Error Code: Unauthorized. Error Number: 6001. Error Message: Unauthorized access.",
    );
  });

  //
  it("update profile: correct authority", async () => {
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
    const [oldNameServicePda] = deriveNameServiceAddress("profile", profileData.username);
    const [newNameServicePda] = deriveNameServiceAddress("profile", new_username);

    const wrongAuthority = anchor.web3.Keypair.generate();

    await expect(
      program.methods
        .changeUsername(random_seed_profile as unknown as number[], new_username)
        .accounts({
          // note: when not provided, Anchor should auto-magically set this to the fee payer
          authority: wrongAuthority.publicKey,
          profile: profilePda,
          oldNameService: oldNameServicePda,
          newNameService: newNameServicePda,
        })
        .signers([wrongAuthority])
        .rpc(),
    ).to.eventually.be.rejectedWith(
      "AnchorError caused by account: profile. Error Code: Unauthorized. Error Number: 6001. Error Message: Unauthorized access.",
    );
  });

  //
  it("change username: correct authority", async () => {
    // set a new usernames
    const new_username = "brand_new_not_taken";

    // derive the profile's name service account from the profile's username

    const [oldNameServicePda] = deriveNameServiceAddress("profile", profileData.username);
    const [newNameServicePda] = deriveNameServiceAddress("profile", new_username);

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
    const updatedProfile = await program.account.profile.fetch(new_name_service.address);

    // ensure the update name service record points to the correct record address
    const [nameServiceTestPda] = deriveNameServiceAddress("profile", updatedProfile.username);
    assert(
      newNameServicePda.toBase58() === nameServiceTestPda.toBase58(),
      "Expected the 'newNameServicePda' to be derived from the new 'updatedProfile.username'",
    );

    // ensure the new name service has the correct data
    assert(
      new_name_service.address.toBase58() === profilePda.toBase58(),
      "Expected 'address' to be the profile pda",
    );

    // ensure the old name service account was closed
    await expect(
      program.account.nameService.fetch(oldNameServicePda),
    ).to.eventually.be.rejectedWith("Account does not exist or has no data");
  });

  // todo: changing profile authority
  // todo: verify post authority is still correct, being the profile pda instead of the `profile.authority`
});

describe("post_group", () => {
  const postGroupName = "test";
  const [nameServicePda] = deriveNameServiceAddress("post_group", postGroupName);

  //
  it("create post group", async () => {
    console.log("\t", "post group address:", postGroupPda.toBase58());

    await program.methods
      .createPostGroup(random_seed_postGroup as unknown as number[], postGroupName)
      .accounts({
        author: profilePda,
        group: postGroupPda,
        nameService: nameServicePda,
      })
      .rpc();

    // get the PostGroup record from the chain
    const group = await program.account.postGroup.fetch(postGroupPda);

    // note: checks for the post group name service are performed next

    assert(
      group.authority.toBase58() === profilePda.toBase58(),
      "Expected 'authority' to be the 'profilePda'",
    );
    assert(group.name === postGroupName, "Expected 'name' to match");
  });

  //
  it("create name service record", async () => {
    console.log("\t", "name service address:", nameServicePda.toBase58());

    // get the name service and group from the blockchain
    const nameService = await program.account.nameService.fetch(nameServicePda);
    const postGroup = await program.account.postGroup.fetch(nameService.address);

    // ensure the name service record points to the correct record address
    const [nameServiceTestPda] = deriveNameServiceAddress("post_group", postGroup.name);
    assert(
      nameServiceTestPda.toBase58() === nameServicePda.toBase58(),
      "Expected the 'nameServicePda' to be derived from the 'postGroup.name'",
    );

    assert(
      nameService.address.toBase58() === postGroupPda.toBase58(),
      "Expected 'address' to be the 'postGroupPda'",
    );
    assert(
      nameService.address.toBase58() === postGroupPda.toBase58(),
      "Expected 'address' to be the 'postGroupPda'",
    );
    assert(
      nameService.authority.toBase58() === profilePda.toBase58(),
      "Expected 'authority' to be the 'profilePda'",
    );
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
