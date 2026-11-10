<p align="center">
    <img alt="confluence.rs" src="https://raw.githubusercontent.com/braycarlson/confluence.rs/main/assets/confluence-rs-mark-cog.svg" width="200">
</p>

<p align="center">
    <em>A Confluence Cloud REST API client for Rust.</em>
</p>

<p align="center">
    <a href="https://github.com/braycarlson/confluence.rs/actions/workflows/ci.yml"><img alt="ci" src="https://img.shields.io/github/actions/workflow/status/braycarlson/confluence.rs/ci.yml?branch=main&amp;style=flat-square&amp;label=ci"></a>
    <a href="https://www.rust-lang.org"><img alt="rust" src="https://img.shields.io/badge/rust-2024-orange.svg?style=flat-square"></a>
    <a href="https://github.com/braycarlson/confluence.rs/blob/main/LICENSE"><img alt="license" src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square"></a>
</p>

<br>

confluence.rs is a Confluence Cloud REST API client for Rust. The 397 operations of the v1, v2, and Automation APIs are generated from Atlassian's OpenAPI specifications. The v1 API is generated beside its v2 replacement, because v1 still holds what v2 lacks, such as CQL search and attachment uploads.
