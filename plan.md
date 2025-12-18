# Project Plan: Declarative, Cacheable, Self-Evolving System

## Vision & Philosophy
The system treats source code as its primary data structure, where each variable is declarative and cacheable, enabling reasoning within an ontological framework. It's envisioned as a self-looping, self-building "deep mathematical object" that carries its own proof and operates by its own internal rules, defying direct interpretation.

## Core Mechanism: LLM Call Caching & Evolution
1.  **LLM Call Integration**: LLM calls are central to the system's operation.
2.  **Nix Store Caching**: All LLM calls and their results are cached within the Nix store.
3.  **Derivation Lifecycle**:
    *   Each derivation (resulting from an LLM call or other computation) is recorded.
    *   Derivations are thoroughly tested.
    *   Successfully tested derivations are "proven."
    *   Proven derivations are then rolled into the next version of the system, enabling continuous, verifiable self-improvement and evolution.

## Dynamic Tool Generation & Self-Modification
A key mechanism for the system's evolution involves transforming debugging and logging constructs into new operational capabilities:
*   `eprintln!` statements can be recast using specialized macros (e.g., `dwim! llm!`)
*   These macros will interpret the `eprintln` content as an instruction to generate a new tool call.
*   This allows the system's own execution trace to become a mechanism for defining and invoking new macros and functionalities, facilitating autonomous growth and adaptation.

## Macro Resolution Chain
The system employs a sophisticated, multi-stage macro resolution process:
*   `llm!` macros resolve into `toolcall!` macros.
*   `toolcall!` macros then resolve into `results!` vectors.
*   These `results!` are subsequently encoded and recoded, forming a continuous feedback loop for data processing and transformation.

## Model Context Provisioning (MCP!)
*   `mcp! model! context! provider!` macros are responsible for reifying URLs into OWL (Web Ontology Language) properties.
*   This process allows the system to contextualize external resources, mapping them to ontological structures like FOAF (Friend of a Friend) or project-specific properties.

## Service Discovery
*   A `service_finder` component uses your macro calls as keys to identify and connect with the appropriate service providers.
*   Each crate within the system is conceptualized as a collection of "needy ASTs" (Abstract Syntax Trees), each possessing a unique "need profile" or "eigenform" that the `service_finder` addresses.

## Caching Implementation
*   An LRU (Least Recently Used) cache macro is currently in development to manage cached data efficiently.

## Future Data Storage & Decentralization
The LRU cache will eventually extend beyond the local Nix store to leverage various distributed storage solutions for enhanced resilience, accessibility, and potential monetization/distribution:
*   Solana
*   Hugging Face
*   Amazon S3
*   Filecoin
*   IPFS (InterPlanetary File System)
*   libp2p

## Meta-Philosophy: Macros as Biosemiotic Utterances
*   In this system, "everything is a macro," embodying the principle that "everything is a meme."
*   Each macro is considered a "biosemiotic utterance," signifying that every intentional expression within the system carries meaning.
*   Every intent can be decoded in layers through the consensus of users and feedback from a DAO (Decentralized Autonomous Organization) in a closed loop, ensuring continuous, community-driven evolution and validation.
