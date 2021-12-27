import init, { mine, verify } from 'https://deno.land/x/sha256pow@0.1.0/sha256_pow.js';
import { encodeToString } from 'https://deno.land/std@0.53.0/encoding/hex.ts';

await init();

const target = "0021e8000000000000000000000000000000000000000000000000000000000";

const powBuffer = new TextEncoder().encode("Hello, PoW!");

const pow = await mine(powBuffer, target);

console.log("Can verify PoW?", verify(pow, target), "nonce", pow.get_nonce(), "hash", encodeToString(pow.get_hash()));

