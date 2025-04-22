# 🌳 URLTree: Efficient URL Pattern Matching

A high-performance URL pattern matching implemented in **Rust** 🚀, designed to store and query URL rules with support for wildcard patterns (`*` and `?`). The algorithm organizes URLs in a tree structure, labeling them as allowed or disallowed, making it ideal for applications requiring fast URL filtering.

## ✨ Features

- **Efficient Tree Structure**: Stores URLs with a label indicating whether they are allowed or disallowed.
- **Wildcard Support**: Handles patterns like `google.com/*/test` or `google.com/*.txt` using `*` (zero or more characters) and `?` (single character).
- **High Performance**: Outperforms optimized Kotlin implementation in both tree construction and pattern matching.

## 📊 Performance

Tested with a dataset of **20,000 URLs**:

- **Tree Construction**:
  - Kotlin: 20 ms
  - Rust: 9 ms ✅
- **URL Pattern Matching**:
  - Kotlin: 18 ms
  - Rust: 12 ms (further optimization possible for string handling) ⚡

> **Note**: The Rust implementation currently uses string creation, which involves allocation/deallocation. Future optimizations (e.g., using a `StringBuilder`-like approach) could further improve performance.

## 🛠️ Use Case Example

URLTree is perfect for applications requiring fast and scalable URL filtering, such as:

### Web Content Filtering

A browser extension or proxy server needs to block or allow access to websites based on predefined rules. For instance:

- Allow `https://example.com/*/public/*`
- Block `*.example.org/*.txt`
- Allow `https://docs.example.com/?/guide`

URLTree can efficiently store these rules and check incoming URLs against them in real-time, ensuring minimal latency even with thousands of rules.

