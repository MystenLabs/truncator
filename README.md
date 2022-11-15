# truncator

<p align="center">
  <img width="300" src="images/truncator_logo.png">
</p>

`truncator` is an experimental benchmarking library exploring compression techniques for cryptographic outputs via 
brute-force mining. It is part of the joint research project entitled <b>"Truncator: Time-space Tradeoff of Cryptographic 
Primitives"</b> by Foteini Baldimtsi (GMU & Mysten Labs), Konstantinos Chalkias (Mysten Labs),
Panagiotis Chatzigiannis (Visa Research), and Mahimna Kelkar (Cornell University).

### Paper
A 10-pager short paper is available here: [Truncator short paper](/truncator_short_paper.pdf "Truncator: Time-space Tradeoff of Cryptographic
Primitives")

### Abstract

We're presenting mining-based techniques to reduce the size of various cryptographic outputs without loss of security. 
Our approach can be  generalized for multiple primitives, such as key generation, signing, hashing and encryption 
schemes, by introducing a brute-forcing step to provers/senders aiming at compressing submitted cryptographic material. 
As a result, in systems that we can tolerate sender's work to be more demanding and time-consuming,
we manage to optimize on verification, payload size and storage cost, especially when:
- receivers have limited resources (i.e. mobile, IoT);
- storage or data-size is financially expensive (i.e. blockchains, cloud storage and ingress cost);
- multiple recipients perform verification/decryption/lookup (i.e. blockchains, TLS certs, IPFS lookups).


Interestingly, mining can result in record-size cryptographic outputs, and we show that 5\%-12\% shorter hash digests 
and signatures are practically feasible even with commodity hardware. Obviously, the first thing that comes to mind is 
compressing addresses and transaction signatures in order to pay less gas fees in blockchain applications, but in fact 
even traditional TLS certificates and public keys, which are computed once and reused in every new connection, can be 
slightly compressed with this _mining_ trick without compromising security. The effects of 
_compressing once - then reuse_ at mass scale can be economically profitable in the long run for both the Web2 and 
Web3 ecosystems.
    
Our paradigm relies on a brute-force search operation in order to craft the primitive's output such that it fits into 
fewer bytes, while the _missing_ fixed bytes are implied by the system parameters and omitted from the actual 
communication. While such compression requires computational effort depending on the level of compression, this cost is 
only paid at the source (typically in blockchains consisting of a single party) which is rewarded by lowered transaction
fees, and the benefits of the compression are enjoyed by the whole ecosystem. As a starting point, we show how our 
paradigm applies to some basic primitives (commonly used in blockchain applications), and show how security is preserved
using a bit security framework. Surprisingly, we also identified cases where wise mining strategies require 
proportionally less effort than naive brute-forcing, an example is WOTS (and inherently SPHINCS) post-quantum signatures
where the target goal is to remove or compress the Winternitz checksum part. Moreover, we evaluate our approach for 
several primitives based on different levels of compression which concretely demonstrates the benefits (both in terms 
of financial cost and storage) if adopted by the community.
    
Finally, as this work is inspired by the recent unfortunate buggy _gas golfing_ software in Ethereum, where weakly 
implemented functions incorrectly generated addresses (hashes) with _prefixed zeroes for gas optimization_, resulting in
millions of losses, we expect our _Truncator_ approach to be naturally applied in the blockchain space as a secure 
solution to more succinct transactions, addresses and states.
