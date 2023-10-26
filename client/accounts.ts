/**
 *
 */

import * as anchor from "@coral-xyz/anchor";
import type { Program } from "@coral-xyz/anchor";
import type { PublicKey } from "@solana/web3.js";
import type { Social } from "../target/types/social";

/**
 * Derive a Profile's PDA address
 */
export function deriveProfileAddress(random_seeds: Uint8Array) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      // comment for better diffs
      Buffer.from("profile", "utf8"),
      random_seeds,
    ],
    anchor.workspace.Social.programId,
  );
}

/**
 * Derive a PostGroup's PDA address
 */
export function derivePostGroupAddress(random_seed: Uint8Array) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      // comment for better diffs
      Buffer.from("post_group", "utf8"),
      random_seed,
    ],
    anchor.workspace.Social.programId,
  );
}

/**
 * Derive a Post's PDA address
 */
export function derivePostAddress(group: PublicKey, post_id: number) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      // comment for better diffs
      Buffer.from("post", "utf8"),
      group.toBytes(),
      Buffer.from(post_id.toString(), "utf-8"),
    ],
    anchor.workspace.Social.programId,
  );
}

type NameSpaceValue = "profile" | "post" | "post_group";

/**
 * Derive a LookupAccount's PDA address
 */
export function deriveLookupAccountAddress(namespace: NameSpaceValue, key: string) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      // comment for better diffs
      Buffer.from("lookup", "utf8"),
      Buffer.from(namespace, "utf8"),
      Buffer.from(key, "utf8"),
    ],
    anchor.workspace.Social.programId,
  );
}
