import init, { mine, mine_random, verify } from 'https://deno.land/x/sha256pow@0.1.5/sha256_pow.js';
import bsv_init, { PrivateKey, ECDSA, SigHash } from "https://deno.land/x/bsv_wasm@deno-optimisations/bsv_wasm.js";
import { encodeToString } from 'https://deno.land/std@0.53.0/encoding/hex.ts';

await bsv_init();
await init();

const priv = PrivateKey.fromRandom();

const target = "0021e8000000000000000000000000000000000000000000000000000000000";

const powBuffer = new TextEncoder().encode("Hello, PoW!");

const pow = await mine_random(powBuffer, target);

const hash = pow.get_hash();
const nonce = pow.get_nonce();
const template = pow.get_template();
console.log("Can verify PoW?", verify(pow, target), "nonce", nonce, "hash", encodeToString(hash));