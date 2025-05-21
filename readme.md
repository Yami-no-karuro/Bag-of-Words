# Rust - Search Engine

## A Simple TF-IDF Search Engine, in Rust

### Introduction

In the field of **Information Retrieval**, the **TF-IDF** method (short for *Term Frequency – Inverse Document Frequency*) is a technique used to evaluate how important a word is within a document, taking into account how frequently that word appears across all documents in a collection.  
It is an enhancement of the classic **Bag-of-Words** model, which represents a document simply as a collection of words, ignoring their order.  
TF-IDF introduces a key idea: not all words are equally informative. Common terms like “the” or “and” appear frequently across documents and therefore provide little value in distinguishing content. In contrast, rarer terms can better identify the topic of a text.  
(More on [Information Retrieval](https://en.wikipedia.org/wiki/Information_retrieval), [TF-IDF](https://en.wikipedia.org/wiki/Tf%E2%80%93idf) and [Bag-of-Words](https://en.wikipedia.org/wiki/Bag-of-words_model) on [Wikipedia](https://en.wikipedia.org))

### What is this project about?

This project is a **from-scratch** implementation of a small TF-IDF-based search engine, written entirely in **Rust**.  
Given a folder containing a set of documents, the engine is able to return the most relevant files for a given textual query, ranking them by **similarity score**.  
Similarity is calculated by comparing the **TF-IDF vectors** of the documents with that of the query, using **cosine similarity** as a distance metric.
(More about [Cosine Similarity](https://en.wikipedia.org/wiki/Cosine_similarity) on [Wikipedia](https://en.wikipedia.org))

### How does it work?

Here's the general flow, step by step:

1. **Tokenization**  
   Each document is split into lowercase words (tokens) to ensure consistency.

2. **Bag-of-Words**  
   A frequency map is created to count how many times each word appears in a document.

3. **Document Frequency (DF)**  
   Across the entire corpus, the engine counts how many documents contain each term.

4. **Inverse Document Frequency (IDF)**  
   The IDF assigns more weight to rare terms and less to common ones.

5. **TF-IDF Vectors**  
   Each document (and the query) is transformed into a vector, where each element represents the TF-IDF weight of a term.

6. **Cosine Similarity**  
   Finally, the query vector is compared with each document vector using cosine similarity, to determine which documents are most relevant.

### Libraries & Honorable mentions

- [Tokenizer](https://github.com/Yami-no-karuro/Tokenizer).

