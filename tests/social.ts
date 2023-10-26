import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import type { Social } from "../target/types/social";

import chai, { expect, assert } from "chai";
import chaiAsPromised from "chai-as-promised";
import {
  deriveLookupAccountAddress,
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

    const [lookupAccountPda] = deriveLookupAccountAddress("profile", profileData.username);

    await program.methods
      .createProfile(profileData)
      .accounts({
        // payer: payer.publicKey,
        authority: payer.publicKey,
        profile: profilePda,
        lookupAccount: lookupAccountPda,
      })
      .rpc();

    // get the profile record from the chain
    const profile = await program.account.profile.fetch(profilePda);

    // note: the lookup account checks for profile creation are next

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
  it("create profile lookup account", async () => {
    // derive the profile's lookup account from the profile's username
    const [lookupAccountPda] = deriveLookupAccountAddress("profile", profileData.username);

    console.log("\t", "lookup account address:", lookupAccountPda.toBase58());

    // get the lookup account and profile from the blockchain
    const lookup_account = await program.account.lookupAccount.fetch(lookupAccountPda);
    const updatedProfile = await program.account.profile.fetch(lookup_account.address);

    // ensure the lookup account points to the correct address
    const [lookupAccountTestPda] = deriveLookupAccountAddress("profile", updatedProfile.username);
    assert(
      lookupAccountPda.toBase58() === lookupAccountTestPda.toBase58(),
      "Expected the 'lookupAccountPda' to be derived from the 'updatedProfile.username'",
    );

    assert(
      updatedProfile.authority.toBase58() === payer.publicKey.toBase58(),
      "Expected 'payer' to be the profile authority",
    );
    assert(
      lookup_account.address.toBase58() === profilePda.toBase58(),
      "Expected 'address' to be the profile pda",
    );
    assert(
      lookup_account.authority.toBase58() === profilePda.toBase58(),
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

    // derive the profile's lookup account from the profile's username
    const [oldLookupAccountPda] = deriveLookupAccountAddress("profile", profileData.username);
    const [newLookupAccountPda] = deriveLookupAccountAddress("profile", new_username);

    const wrongAuthority = anchor.web3.Keypair.generate();

    await expect(
      program.methods
        .changeUsername(random_seed_profile as unknown as number[], new_username)
        .accounts({
          // note: when not provided, Anchor should auto-magically set this to the fee payer
          authority: wrongAuthority.publicKey,
          profile: profilePda,
          oldLookupAccount: oldLookupAccountPda,
          newLookupAccount: newLookupAccountPda,
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

    // derive the profile's lookup account from the profile's username

    const [oldLookupAccountPda] = deriveLookupAccountAddress("profile", profileData.username);
    const [newLookupAccountPda] = deriveLookupAccountAddress("profile", new_username);

    console.log("\t", "new lookup account address:", newLookupAccountPda.toBase58());

    await program.methods
      .changeUsername(random_seed_profile as unknown as number[], new_username)
      .accounts({
        // note: when not provided, Anchor should auto-magically set this to the fee payer
        // authority: payer.publicKey,
        profile: profilePda,
        oldLookupAccount: oldLookupAccountPda,
        newLookupAccount: newLookupAccountPda,
      })
      .rpc();

    // get the updated profile record from the chain
    const new_lookup_account = await program.account.lookupAccount.fetch(newLookupAccountPda);
    const updatedProfile = await program.account.profile.fetch(new_lookup_account.address);

    // ensure the update lookup account points to the correct record address
    const [lookupAccountTestPda] = deriveLookupAccountAddress("profile", updatedProfile.username);
    assert(
      newLookupAccountPda.toBase58() === lookupAccountTestPda.toBase58(),
      "Expected the 'newLookupAccountPda' to be derived from the new 'updatedProfile.username'",
    );

    // ensure the new lookup account has the correct data
    assert(
      new_lookup_account.address.toBase58() === profilePda.toBase58(),
      "Expected 'address' to be the profile pda",
    );

    // ensure the old lookup account was closed
    await expect(
      program.account.lookupAccount.fetch(oldLookupAccountPda),
    ).to.eventually.be.rejectedWith("Account does not exist or has no data");
  });

  // todo: changing profile authority
  // todo: verify post authority is still correct, being the profile pda instead of the `profile.authority`
});

describe("post_group", () => {
  const postGroupName = "test";
  const [lookupAccountPda] = deriveLookupAccountAddress("post_group", postGroupName);

  //
  it("create post group", async () => {
    console.log("\t", "post group address:", postGroupPda.toBase58());

    await program.methods
      .createPostGroup(random_seed_postGroup as unknown as number[], postGroupName)
      .accounts({
        author: profilePda,
        group: postGroupPda,
        lookupAccount: lookupAccountPda,
      })
      .rpc();

    // get the PostGroup record from the chain
    const group = await program.account.postGroup.fetch(postGroupPda);

    // note: checks for the post group lookup account are performed next

    assert(
      group.authority.toBase58() === profilePda.toBase58(),
      "Expected 'authority' to be the 'profilePda'",
    );
    assert(group.name === postGroupName, "Expected 'name' to match");
  });

  //
  it("create lookup account record", async () => {
    console.log("\t", "lookup account address:", lookupAccountPda.toBase58());

    // get the lookup account and group from the blockchain
    const lookupAccount = await program.account.lookupAccount.fetch(lookupAccountPda);
    const postGroup = await program.account.postGroup.fetch(lookupAccount.address);

    // ensure the lookup account points to the correct record address
    const [lookupAccountTestPda] = deriveLookupAccountAddress("post_group", postGroup.name);
    assert(
      lookupAccountTestPda.toBase58() === lookupAccountPda.toBase58(),
      "Expected the 'lookupAccountPda' to be derived from the 'postGroup.name'",
    );

    assert(
      lookupAccount.address.toBase58() === postGroupPda.toBase58(),
      "Expected 'address' to be the 'postGroupPda'",
    );
    assert(
      lookupAccount.address.toBase58() === postGroupPda.toBase58(),
      "Expected 'address' to be the 'postGroupPda'",
    );
    assert(
      lookupAccount.authority.toBase58() === profilePda.toBase58(),
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
        group: postGroupPda,
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
