# Web Workers with WebAssembly

Demonstrates using WebAssembly in Web Workers for CPU-intensive tasks without blocking the main thread.

## Concepts Covered

- Web Workers for parallel processing
- CPU-intensive computations in WASM
- Message passing between main thread and workers
- Performance monitoring
- Non-blocking UI with background tasks

## Why Web Workers?

Web Workers allow you to run JavaScript (and WASM) in background threads, preventing heavy computations from freezing the UI.

**Benefits:**
- Keep UI responsive during heavy computations
- Utilize multiple CPU cores
- Better user experience for compute-intensive apps

## Code Examples

### CPU-Intensive Functions

```rust
#[wasm_bindgen]
pub fn calculate_primes(n: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    for num in 2..=n {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

#[wasm_bindgen]
pub fn estimate_pi(iterations: u32) -> f64 {
    let mut inside_circle = 0;
    for _ in 0..iterations {
        let x = js_sys::Math::random();
        let y = js_sys::Math::random();
        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }
    4.0 * (inside_circle as f64) / (iterations as f64)
}
```

## Building

```bash
wasm-pack build --target web
```

## Usage with Web Workers

### Worker Script (worker.js)

```javascript
import init, { calculate_primes, estimate_pi } from './pkg/web_workers.js';

let initialized = false;

self.addEventListener('message', async (event) => {
    if (!initialized) {
        await init();
        initialized = true;
    }

    const { task, data } = event.data;

    let result;
    const start = performance.now();

    switch (task) {
        case 'primes':
            result = calculate_primes(data.n);
            break;
        case 'pi':
            result = estimate_pi(data.iterations);
            break;
        default:
            result = null;
    }

    const elapsed = performance.now() - start;

    self.postMessage({
        task,
        result,
        time_ms: elapsed
    });
});
```

### Main Thread (index.html)

```html
<!DOCTYPE html>
<html>
<head>
    <title>Web Workers + WASM</title>
</head>
<body>
    <h1>Web Workers with WebAssembly</h1>

    <button id="primesBtn">Calculate Primes</button>
    <button id="piBtn">Estimate Pi</button>

    <div id="results"></div>
    <div id="status">Ready</div>

    <script type="module">
        const worker = new Worker('worker.js', { type: 'module' });
        const results = document.getElementById('results');
        const status = document.getElementById('status');

        worker.addEventListener('message', (event) => {
            const { task, result, time_ms } = event.data;
            results.innerHTML += `<p>${task}: ${result} (${time_ms.toFixed(2)}ms)</p>`;
            status.textContent = 'Ready';
        });

        document.getElementById('primesBtn').addEventListener('click', () => {
            status.textContent = 'Calculating primes...';
            worker.postMessage({
                task: 'primes',
                data: { n: 100000 }
            });
        });

        document.getElementById('piBtn').addEventListener('click', () => {
            status.textContent = 'Estimating Pi...';
            worker.postMessage({
                task: 'pi',
                data: { iterations: 10000000 }
            });
        });
    </script>
</body>
</html>
```

## Multiple Workers

For better parallelization, use multiple workers:

```javascript
const numWorkers = navigator.hardwareConcurrency || 4;
const workers = [];

for (let i = 0; i < numWorkers; i++) {
    const worker = new Worker('worker.js', { type: 'module' });
    worker.addEventListener('message', handleResult);
    workers.push(worker);
}

// Distribute work
function calculatePrimes(max) {
    const chunkSize = Math.floor(max / numWorkers);

    workers.forEach((worker, i) => {
        const start = i * chunkSize;
        const end = i === numWorkers - 1 ? max : (i + 1) * chunkSize;

        worker.postMessage({
            task: 'primes-range',
            data: { start, end }
        });
    });
}
```

## Performance Comparison

### Main Thread vs Worker

```javascript
// Main thread (blocks UI)
const primes = await init().then(() => calculate_primes(100000));
// UI is frozen during computation

// Web Worker (non-blocking)
worker.postMessage({ task: 'primes', data: { n: 100000 } });
// UI remains responsive
```

## Best Practices

1. **Initialize Once**: Initialize WASM module once in worker
2. **Batch Work**: Send larger chunks of work to minimize message overhead
3. **Shared Array Buffers**: For large data transfer (requires COOP/COEP headers)
4. **Error Handling**: Always handle worker errors
5. **Terminate Workers**: Clean up workers when done

## Error Handling

```javascript
worker.addEventListener('error', (event) => {
    console.error('Worker error:', event.message);
});

worker.addEventListener('messageerror', (event) => {
    console.error('Message error:', event);
});
```

## Limitations

- Cannot access DOM from workers
- Message passing has overhead for large data
- No shared memory (without SharedArrayBuffer)
- Requires CORS for worker scripts

## Use Cases

- **Scientific Computing**: Simulations, data analysis
- **Image Processing**: Filters, transformations
- **Cryptography**: Hashing, encryption
- **Game Logic**: Physics, AI pathfinding
- **Data Processing**: Sorting, searching large datasets

## References

- [Web Workers API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API)
- [Using Web Workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers)
- [SharedArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)
