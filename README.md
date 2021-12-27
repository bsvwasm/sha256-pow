# sha256-pow

```js
import init, { mine, verify } from 'https://deno.land/x/sha256pow@0.1.0/sha256_pow.js';

await init();

const target = "00000021e800000000000000000000000000000000000000000000000000000";

const powBuffer = new TextEncoder().encode("Hello, PoW!");

const pow = await mine(powBuffer, target);

console.log("Can verify PoW?", verify(pow, target));
```
