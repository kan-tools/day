---
{
  "v": 3,
  "cid": "bafyreihlpjpnhf7ps3z7sttnkdt7hpgtncipuaipdfey5ddfwydtslxute",
  "sig": "ed42fca5ee4ec8fd41c9b925ce59b40a9019cb2b7e6d5563ecfd5026597552da72f0148c40b1acbc7184fc41091b4fd4925d5d96f85295751cda20a3e043708f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mt33hjfkur",
  "seq": 0,
  "of": 31,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoNWQ2YzBjYzhmODcxMDllMjUxYjkxM2JjN2YyYWE4ZDNlN2ZjYmUzZml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkIWvXC3w=="
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 14297:6f68e8976baa67a5]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee",
  "sig": "e3ed23a0d0ba8f5eabe02618531666772f0b4f55fa8cdbb851be703bc00714c32590ed88680f2116645354292d3ee7fe8bf6c37ca5bfd75be771f6a6897f2bcb",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreihlpjpnhf7ps3z7sttnkdt7hpgtncipuaipdfey5ddfwydtslxute"
  ],
  "rev": "223mt33hjsnxb",
  "seq": 1,
  "of": 31,
  "text_len": 659,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDrel7Tl++W8/lObVDn87zTaJD6AQ8ZSY6MZbYHOS70mWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDVkNmMwY2M4Zjg3MTA5ZTI1MWI5MTNiYzdmMmFhOGQzZTdmY2JlM2Zpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZCFr8Tzg="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13` makes ongoing human-directed work inspectable without turning day into a task tracker or a transcript. It first repairs two places where the record currently overstates its evidence, then ships a stream overview, a general `/askme` driver affordance for semi-structured human input, and explicit human intervention events. The release is complete only after those capabilities are used together through a real work cycle and a later session can reconstruct what happened from the durable record. [validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibpydyzf45cwrvg4w6d6gzbtgd6fk5wtka3l2ly67ni5ikk7y6u7u",
  "sig": "a0b8330a840c44013697fefbed4cc55c6b5bfe57e7921acd82b13b33813d402c06b1b8c3ec00ceda8b1f45696eb91c247736782faae5b38827405d7c95401e70",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt33hk3tn5",
  "seq": 2,
  "of": 31,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgiUmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljc2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQhbAOX4"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiboqhgubi7axdu2nsegnp3bzzkeyrgxtksaqvyxocvjh6tntc75wa",
  "sig": "3afab71745c3b39e5edce1f52148f63b5789d676e6630b2e8fdc355f79404f054f9290f34e7720db914e70b8924bced147bc73634d67bfe4c7133fc148602b17",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee"
  ],
  "rev": "223mt33hkrve2",
  "seq": 3,
  "of": 31,
  "text_len": 177,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBo2dfogMpKZZoImXZP6jPoUjIAXdxZrGaaSanScOiyFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQhbC+zO"
}
---

RQ-1: The cycle ships both the complete workflow-visibility set—stream view, general `/askme`, and intervention events—and the two evidence-correctness fixes they depend on.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifu6znrxcd2uod4oh2lkhebaz5osg4hkcfqpylpunv4y4dt33ihuq",
  "sig": "a2448b4cb9b2f88f2f76b27a1d2ddf82c709a98496bb6dad68fb463ef81468a3665b81dccb031ade9f6231f8aeacd220cb7cf4abeaadad9c6ac00e4b14b5f1b6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee"
  ],
  "rev": "223mt33hl6t36",
  "seq": 4,
  "of": 31,
  "text_len": 111,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBo2dfogMpKZZoImXZP6jPoUjIAXdxZrGaaSanScOiyFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQhbEmOt"
}
---

RQ-2: #196 and #152 are release blockers and land before the new recording affordances or their adoption proof.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihxww5su5wsgelsqvjempidni67yrwenibkfdo22hbzcbcqy544be",
  "sig": "083386756c88dfd0bec4f3efcb114a0370b1775506ce7335dd125274c45b15b327b1e37ac05d59dbc18f5d640019663ee67828bb044c1c74b55faf297b76e49a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee"
  ],
  "rev": "223mt33hllqag",
  "seq": 5,
  "of": 31,
  "text_len": 100,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBo2dfogMpKZZoImXZP6jPoUjIAXdxZrGaaSanScOiyFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQhbGNhV"
}
---

RQ-3: Success requires a real-cycle dogfood and later reconstruction, not feature-level tests alone.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieszzez4xrrxnwzt6dalfdlrmzqrbvtaowmcpnsp734yivb65np2i",
  "sig": "8dda16d5d082716988025d2a436f38fa532bf634c90049d76239b49423da97d3791edbc55b50e9dc54d81536a19228f486e5d20d1a3ebc42b03f306976a917c1",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee"
  ],
  "rev": "223mt33hlyo75",
  "seq": 6,
  "of": 31,
  "text_len": 191,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBo2dfogMpKZZoImXZP6jPoUjIAXdxZrGaaSanScOiyFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQhbH1Ar"
}
---

RQ-4: `/askme` is a general facility for convenient semi-structured information acquisition from a human who detects the need; it is not tied to issue resolution or a particular process flow.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibha74vsedeqa6dfz57y5vdp3kievrmbq35e2h7gc2xnhqf2ibqje",
  "sig": "71ca6b9dd8e4bc88cc7cfbea999b487388956925aee8d69a9d4b2e4c8ff6a3f2722c8f3690178b6b27564d7113fa6ddecbc138505bf79fe9e90313558dcc01da",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee"
  ],
  "rev": "223mt33hmfm6r",
  "seq": 7,
  "of": 31,
  "text_len": 261,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBo2dfogMpKZZoImXZP6jPoUjIAXdxZrGaaSanScOiyFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQhbJcgd"
}
---

RQ-5: Trigger-scoped practice injection (#198) and the design-integrity and vocabulary cluster (#200–203) are deferred. Their designs should wait for the forthcoming kan identity and data-model changes rather than freezing assumptions that are already moving.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiakkusc2vsxtq5x62o4gemiee532vqlc5ayjy27sap36crtxqjdy4",
  "sig": "7dace4c25767b21ef07ed2bf11e253f9c076622c4f9dbb7aee57e3485c9330f67249be94c81c7e3d66b0bb4078bff5a7cb147a9ae8cc23d895e1e42d3de2b694",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreihlpjpnhf7ps3z7sttnkdt7hpgtncipuaipdfey5ddfwydtslxute"
  ],
  "rev": "223mt3eidzgkh",
  "seq": 8,
  "of": 31,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg63pe05fvlvP5Tm1Q5/O802iQ+gEPGUmOjGW2Bzku9JlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQqcn7GZ"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 23162:26d2db7dd9b08177]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifk26vwrgfoizig3loe5vw7op4xoedmauwitg4r7pr22omc2hbz7i",
  "sig": "926cfadd99b5ea43d00797b02efb1be4a2721c1b4da68736bae0a9f10a0ec47d594c4faf5fa5f02df99b639a81900f6e66933ca367fb8c054be1f5e6bf4cceb6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiakkusc2vsxtq5x62o4gemiee532vqlc5ayjy27sap36crtxqjdy4",
    "bafyreiagrwox5camustftiejs5sp5iz6qursabo5ywnmm2netkosodulee"
  ],
  "rev": "223mt3eiefkye",
  "seq": 9,
  "of": 31,
  "text_len": 659,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiAKVSQtVlecO39p3DEYghO71WCxdBhONfkB+/CjO8Ejx9gqWCUAAXESIAaNnX6IDKSmWaCJl2T+oz6FIyAF3cWaxmmkmp0nDoshZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoNWQ2YzBjYzhmODcxMDllMjUxYjkxM2JjN2YyYWE4ZDNlN2ZjYmUzZml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkKnKXDWw=="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13` makes ongoing human-directed work inspectable without turning day into a task tracker or a transcript. It first repairs two places where the record currently overstates its evidence, then ships a stream overview, a general `/askme` driver affordance for semi-structured human input, and explicit human intervention events. The release is complete only after those capabilities are used together through a real work cycle and a later session can reconstruct what happened from the durable record. [validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigrevn3jh2ksu22ieu5vb22yg25rnhb6pykab4yzzcyuofk4dh5p4",
  "sig": "d0f227368cec19a0650d37d3a25171e47fed3de6e2b54595ecdb0f2b2300fdba0b84bdbb45dfdf32e2745bf3ab9ccb8873b8cdd34fe442359f64f2d54da43031",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt3eieoimu",
  "seq": 10,
  "of": 31,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgiUmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljc2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQqcqjnr"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihvbfqfwbx3dphoxrjcrwvb5rf2ujdmmdbvrxfqe34nih2pml3qhy",
  "sig": "c321824605785e45c8238779b1396ab85732069810dfbf508471dd8aa9e96a9b53613a056d55bfdaa60eae0baa883073493932bd4ef614cefee23aa13fde3dd2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreifk26vwrgfoizig3loe5vw7op4xoedmauwitg4r7pr22omc2hbz7i"
  ],
  "rev": "223mt3emmel4y",
  "seq": 11,
  "of": 31,
  "text_len": 23162,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCq16tomK5GUG2txO1t9z+XcQbAUsiZuR++OtOYLRw5+mZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDVkNmMwY2M4Zjg3MTA5ZTI1MWI5MTNiYzdmMmFhOGQzZTdmY2JlM2Zpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZCqUlQ+I="
}
---

# Release: v0.13 workflow ergonomics

## Summary

`v0.13` makes ongoing human-directed work inspectable without turning day into
a task tracker or a transcript. It first repairs two places where the record
currently overstates its evidence, then ships a stream overview, a general
`/askme` driver affordance for semi-structured human input, and explicit human
intervention events. The release is complete only after those capabilities are
used together through a real work cycle and a later session can reconstruct
what happened from the durable record.

This work serves `telos/legible-process` by making active work and meaningful
human redirections reconstructable, `telos/honest-reads` by preserving actual
authorship and measurement scope, `telos/affordance-not-enforcement` by keeping
all driver affordances optional, and `telos/v1.0` by reducing dependence on the
author's memory.

## Requirements

- REQ-1: Before new recording affordances ship, `day design record` must stop
  representing mixed agent exploration and human decisions as though one
  identity authored the entire chain (#196). The recorded claims must preserve
  actual authorship under kan's supported identity model without fabricating a
  second source of identity state in day.

- REQ-2: `/handoff` must state the immutable scope of every time-relative
  measurement it records, including the commit or explicit range for suite,
  census, and CI claims; `/wakeup` must distinguish a scoped measurement from
  an older unscoped claim rather than silently re-running the latter against a
  different tree (#152).

- REQ-3: `day stream list` must derive every `agents/handoff/*` thread from one
  bulk kan read and report the thread name, newest live handoff timestamp, and
  a concise preview sufficient to choose a thread (#204). It must not claim a
  worktree, branch, inferred process position, or staleness fact that its inputs
  cannot establish.

- REQ-4: The shipped handoff and wakeup skills must consume the derived stream
  view for list operations rather than independently describing or performing
  their own folds over `kan show --all --json` (#204).

- REQ-5: `/askme` must be a general driver affordance for occasions when a
  human detects a need to provide semi-structured input and wants the agent to
  acquire it conveniently (#193). Its contract must not be limited to issue
  triage, open-subject resolution, or any one atom or process flow.

- REQ-6: `/askme` must establish the topic and available context, ask one
  decision-shaped or information-shaped question at a time, adapt subsequent
  questions to prior answers, allow the human to stop or skip, and summarize
  the acquired information without claiming that a decision was reached when
  none was.

- REQ-7: `/askme` may use repository and kan context available to the acting
  agent, but its first release uses shipped prompting and writes no automatic
  kan claim merely because a conversation occurred. Recording a resulting
  decision or intervention remains an explicit, attributable act.

- REQ-8: day must provide an explicitly invoked convention for recording a
  meaningful human intervention as an ordinary kan observation (#195). The
  event must identify its kind, the work context, what changed because of the
  intervention, and the author who classified it as an intervention.

- REQ-9: An intervention is narrower than a human turn: it records a correction,
  missing context, answer unavailable to the process, stop, or approval that
  materially changed or enabled the work. Day must never infer interventions
  automatically, log every human message, or treat absence of an intervention
  claim as evidence that none occurred.

- REQ-10: Stream listing, `/askme`, and intervention recording must remain
  harness-agnostic at their semantic boundary. Harness adapters may expose the
  skill or render its output, but Claude Code must not become the source of
  truth for the capability or its data.

- REQ-11: The work should land as separately reviewable changes ordered as:
  authorship correctness (#196), handoff measurement scope (#152), stream view
  (#204), general `/askme` (#193), and intervention events (#195). A later
  change may consume an earlier interface, but the release boundary requires
  every disposition to be merged and green.

- REQ-12: Before release, one real project cycle must use the shipped surfaces
  to select an active thread through the stream view, acquire semi-structured
  human input through `/askme`, explicitly record at least one qualifying
  intervention when one genuinely occurs, and hand off. A fresh later session
  must reconstruct the thread, acquired input's effect, intervention, and
  scoped verification claims without relying on the original conversation.

## Acceptance Criteria

- [ ] AC-1: (REQ-1) A multi-identity integration fixture records agent-produced
      exploration and human-resolved questions, then folds the subject under
      each identity and proves every claim appears under its actual author. The
      test fails when the whole chain is signed by either single identity.

- [ ] AC-2: (REQ-1) The implementation obtains identities through kan's public
      identity interface, and a conformance test against every supported kan
      matrix row fails if the required identity operation is unavailable or
      changes shape.

- [ ] AC-3: (REQ-2) A handoff fixture records suite, census, and CI statements
      with immutable commit SHAs, run identifiers, or explicit ranges. After
      the branch is merged and HEAD advances, wakeup rechecks the original
      scopes and does not substitute its new implicit range.

- [ ] AC-4: (REQ-2) A legacy unscoped measurement fixture renders as
      uncheckable with the missing-scope reason; it cannot render as confirmed
      merely because the same command succeeds against the current tree.

- [ ] AC-5: (REQ-3, REQ-4) Hermetic kan output containing multiple handoff
      subjects, superseded claims, retractions, and an unrelated subject yields
      exactly one row per live thread from the newest live handoff. Command and
      skill conformance tests prove both list affordances use this derived view.

- [ ] AC-6: (REQ-3) With no worktree or branch evidence in kan, `day stream
      list` omits those fields and labels its output as recorded handoff state;
      a mutation that introduces inferred process position makes the test fail.

- [ ] AC-7: (REQ-5, REQ-6) Skill scenarios cover a decision request, a factual
      context-gathering request, an unknown initial scope, a skipped question,
      and an early stop. Each scenario asks one question at a time and produces
      a summary that distinguishes supplied facts, decisions, and unresolved
      items.

- [ ] AC-8: (REQ-5) A plugin-conformance fixture invokes `/askme` with no open
      GitHub issue, no open kan subject, and no active day atom and still
      completes a useful semi-structured information-acquisition exchange.

- [ ] AC-9: (REQ-7) Tests prove the initial `/askme` content has no dependency
      on `schema/askme` and that completing the skill without an explicit
      recording instruction appends no kan claims.

- [ ] AC-10: (REQ-8, REQ-9) Intervention fixtures record each supported kind
      with context and effect, reject an empty effect, preserve the recording
      identity, and retrieve the event through the documented retrospective
      read.

- [ ] AC-11: (REQ-8, REQ-9) Hook and skill conformance tests prove no session,
      prompt, or tool event automatically emits an intervention claim and no
      report equates an empty intervention result with proof of zero human
      interventions.

- [ ] AC-12: (REQ-10) The plugin contract and documentation expose the same
      stream, `/askme`, and intervention semantics to Agent Skills consumers;
      tests fail if the only definition or invocation path lives in a
      Claude-specific hook, command, or metadata file.

- [ ] AC-13: (REQ-11) Each of #196, #152, #204, #193, and #195 cites a merged
      disposition and green required checks. The release script refuses the
      cut if any disposition or required gate is absent.

- [ ] AC-14: (REQ-12) A checked-in adoption artifact identifies the exact day
      commit, kan version, handoff thread, `/askme` purpose, intervention claim
      CID, scoped handoff claim, and later wakeup result. The artifact fails
      validation if any reference cannot be re-read or if reconstruction needs
      an unrecorded chat transcript.

## Architecture

### Correctness before new affordances

The first two changes repair the evidence substrate used by the rest of the
cycle. `day design record` currently emits an `observe`, a `plan`, and human
resolved-question `decide` claims through one active identity. The fix belongs
at the boundary that invokes kan rather than in a day-owned identity registry;
its exact CLI/API shape must follow the supported kan identity model available
when implementation begins. `skills/design/SKILL.md` and the recording path in
`src/` must describe the same authorship split.

The handoff pair in `skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md`
must carry immutable measurement coordinates. Existing handoffs remain readable
historical claims, but the reader reports missing scope honestly. This change
must land before the cycle's dogfood handoff so the trial itself produces
evidence a later session can re-run.

### One derived view of recorded streams

`day stream list` is a read-only fold over `agents/handoff/*`, using the same
bulk-read and live-claim semantics as existing kan-backed readers. The CLI
implementation belongs in the existing verb dispatch under `src/`; its output
model should be reusable by skill-facing rendering rather than duplicated in
`skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md`. It reports what the
record contains. Richer worktree-aware stream identity remains separate because
the current log cannot establish it.

### `/askme` as a general driver affordance

`skills/askme/SKILL.md` is interaction policy, not a new process atom and not a
wrapper around `kan issues`. It accepts a human-supplied topic or begins by
eliciting one, gathers relevant local context, and conducts an adaptive
one-question-at-a-time interview. Open subjects may be useful context in one
invocation, but they are neither a prerequisite nor the semantic input.

The skill separates acquired facts, human decisions, and unresolved items in
its summary. It can recommend an existing explicit recording operation when the
exchange produced something durable, but it does not auto-record the exchange.
The first release deliberately ships fixed interaction guidance; declarable
prompts wait for the shared declared-preference layer.

### Explicit intervention events

The intervention surface is a calling convention over ordinary kan
observations, not a new kan claim kind and not stored day state. Its subject and
block schema must make events retrievable from the work context without relying
on inbound citation traversal. The implementation may be a small day verb, a
skill, or a composition of both, but the durable form and retrieval semantics
must be identical across harnesses.

Classification remains human-directed. The implementation validates that the
operator supplied a kind, context, and material effect; it cannot validate that
the classification is philosophically correct, and therefore never treats the
resulting corpus as exhaustive. Retrospectives may say “recorded interventions”
and must not say “all interventions.”

### Release sequencing and adoption proof

The five issue dispositions remain independently reviewable. #196 and #152
land first because later recording and dogfood evidence depend on them. #204,
#193, and #195 can then land separately, with shared plugin-conformance changes
kept in the owning pull request rather than accumulated into a release-only
patch.

The adoption trial is not a scripted happy-path demonstration. It uses a real
thread and a real need for human input. `/askme` may acquire information wholly
unrelated to issue triage; its evidence is that the human could provide
semi-structured input efficiently and that the resulting effect was summarized
honestly. An intervention is recorded only if one actually occurs. If none
occurs, the trial must continue into another real work segment rather than
manufacturing one to satisfy the criterion. The later wakeup is the decisive
check: it must reconstruct the work from kan, git, and CI references without
the original chat.

## Delivery Plan

### Phase 0: Preserve and challenge the plan

Create a dedicated `v0.13-workflow-ergonomics` branch containing this roadmap
as its first isolated change. Run a cold adversarial review against
`telos/v0.13-workflow-ergonomics` before implementation and disposition every
finding. A blocking finding changes this document before a feature branch
begins; the review is not bundled into the first implementation PR.

Resolve the recorded `bridge/v0.13-workflow-ergonomics` failure. The current
`design > generative-build > adversarial-review > pull-request > release`
sequence ends at `published-artifact`, while the release telos also requires a
`workflow-reconstruction-trial`. Add an explicit, checkable trial step to the
declared project vocabulary with that output and extend the bridge through it.
Do not rename `published-artifact` or weaken the telos to make the old chain
pass.

Exit gate: the roadmap is committed alone, the cold review has no unaccounted
blocking findings, and `day bridge check v0.13-workflow-ergonomics` reaches the
author-run release telos.

### Phase 1: Authorship correctness (#196)

Trace chain construction in `src/design.rs`, identity and environment handling
in `src/kan_client.rs`, CLI binding in `src/cli/mod.rs`, and the contract in
`skills/design/SKILL.md` and `docs/CONVENTIONS.md`. Capture kan's supported
public identity operation in the compatibility fixtures before choosing a CLI
shape; day must not grow its own role registry.

Implement the smallest split that leaves the exploration `Observation` and
design `Plan` under the acting agent while signing each human-resolved
`Decision` under the human identity. Add the multi-key fold proof to
`tests/design.rs` and supported-version coverage to
`tests/kan_conformance.rs`. A negative fixture that signs the whole chain with
either one identity must fail AC-1.

PR gate: focused tests, `cargo test --workspace --no-fail-fast`, kan matrix and
plugin conformance, a cold adversarial review, zero unaccounted findings, and a
merged disposition on #196.

### Phase 2: Immutable handoff measurements (#152)

Define measurement coordinates once in `skills/handoff/SKILL.md`: commit SHA
for a local suite, explicit base and head SHAs for range-based censuses, and CI
run identifier plus head SHA. Mirror the contract in
`skills/wakeup/SKILL.md`: scoped claims are rechecked at their recorded scope;
legacy unscoped measurements are `UNCHECKABLE` with the absent coordinate
named.

Build a round-trip fixture in `tests/plugin.rs` or a dedicated integration
target. It writes a handoff on a topic branch, advances or merges HEAD, and
proves wakeup retains the original scope. A legacy fixture must fail if a
successful current-tree command is reported as confirmation of an unscoped
historical claim.

PR gate: the round trip survives HEAD movement, documented invocations and
failed-read handling remain valid, the full/plugin suites pass, review findings
are dispositioned, and #152 has a merged disposition.

### Phase 3: One recorded-stream view (#204)

Add a reusable stream fold alongside the existing kan-backed read models in
`src/`, expose it through `src/cli/mod.rs` as `day stream list`, and source it
from one bulk read through `src/kan_client.rs`. Its model is limited to the
handoff subject suffix, newest live claim time, live claim count, and a bounded
preview. It does not call `day status`, inspect arbitrary worktrees, or infer
branch and staleness state absent from the claim.

Cover multiple threads, supersession, retraction, unrelated subjects, empty
logs, and malformed input in a dedicated integration target. Change the
`--list` paths in `skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md` to call
the verb; `tests/documented_invocations.rs` and `tests/plugin.rs` prove the
skills no longer contain independent folds.

PR gate: one implementation produces the CLI and both skill views, output
remains honest without worktree evidence, full/plugin suites pass, review
findings are dispositioned, and #204 has a merged disposition.

### Phase 4: General `/askme` affordance (#193)

Add `skills/askme/SKILL.md` as a non-atom driver skill and wire it through the
plugin discovery checked by `tests/agent_plugins.rs` and `tests/plugin.rs`.
Narrowly update the existing “every skill is an atom” invariant to distinguish
driver affordances from process atoms; do not invent an artifact merely to
satisfy composition.

Exercise interaction scenarios for decision-shaped input, fact gathering,
unknown initial topic, skip, early stop, and a repository with no issue, open
kan subject, or active atom. Each asks one question at a time and distinguishes
facts, decisions, and unresolved items. A kan-log fixture snapshots claim count
before and after and proves `/askme` alone writes nothing. No `schema/askme`
loader ships.

PR gate: Agent Skills and Claude-facing discovery agree, the no-context case is
useful, no implicit record is written, full/plugin suites pass, review findings
are dispositioned, and #193 has a merged disposition.

### Phase 5: Explicit intervention events (#195)

Specify one cross-harness `day-intervention` block carried by an ordinary kan
`Observation`. Choose a subject shape directly retrievable from the work
context rather than relying only on inbound citation traversal. The explicit
recording surface validates a supported kind, context, and non-empty material
effect, preserves the invoking identity, and prints the CID. Reads consistently
say “recorded interventions,” never “all interventions.”

Place parsing and validation at the existing boundaries in `src/blocks.rs`,
`src/record.rs`, and `src/cli/mod.rs` as appropriate. Tests cover every initial
kind, empty-effect rejection, retrieval, identity preservation, and the absence
of automatic writes from hooks, prompts, sessions, and `/askme`.

PR gate: explicit invocation is the only write path, empty results make no
exhaustiveness claim, full/plugin suites pass, review findings are
dispositioned, and #195 has a merged disposition.

### Phase 6: Integrate and preregister the reconstruction trial

After all five changes merge, run required workflows on the exact candidate
SHA. Update `scripts/cut-release.sh` and guards in
`tests/harness_honesty.rs` so the v0.13 issue set is the release disposition
gate. Keep issue implementation and conformance changes in their owning PRs;
the integration change contains only cross-cutting trial and release wiring.

Check in the trial protocol and negative controls before dogfooding. It fixes
the candidate commit, kan version, handoff thread, selection through `day
stream list`, `/askme` purpose, qualifying intervention definition, scoped
handoff fields, later-session environment, and expected reconstruction fields.
Controls fail when the intervention CID is absent or unreadable, a measurement
lacks scope, the wrong thread is selected, or reconstruction needs the chat.

Exit gate: preregistration predates evidence, all five issues have merged
dispositions and green checks, and the candidate SHA is frozen except for
trial-discovered fixes.

### Phase 7: Run the real cycle and reconstruct it cold

Use the candidate in real work. Select its thread through the stream view,
invoke `/askme` for a genuine semi-structured need, and record an intervention
only when one qualifies. If none occurs, continue real work rather than
manufacturing one. Write a handoff with immutable suite, census, and CI scopes.

Start a fresh later session without the original transcript. It reconstructs
the chosen thread, what the acquired information changed, the intervention and
its effect, and every verification scope. Record the Result under the
preregistered witness subject, then recheck the bridge and author-run telos.

Failure gate: a missing reference, unverifiable scope, transcript dependency,
or negative control that passes returns the affected feature to its owning
phase and another review. The independent non-author trial may run concurrently
or afterward, but does not block v0.13.

### Phase 8: Cut, publish, and assess v0.13

Run `scripts/cut-release.sh` from clean `main` at the exact trialed commit. It
must verify origin synchronization, all five issue dispositions, the full
suite, compatibility and migration rows, block corpus, plugin manifests, and
the reconstruction witness before producing the release commit and tag. Let
`.github/workflows/release.yml` publish the tag, then verify the release and
required workflows at that SHA.

Record the post-release documentation assessment, run the telos assessment,
and capture follow-ups without rewriting the trial. The cycle closes only when
the artifact is installable, the reconstruction witness remains readable, the
bridge reaches the telos, and a final handoff names exact release, CI, suite,
and assessment coordinates.

## Resolved Questions

- RQ-1: The cycle ships both the complete workflow-visibility set—stream view,
  general `/askme`, and intervention events—and the two evidence-correctness
  fixes they depend on.
- RQ-2: #196 and #152 are release blockers and land before the new recording
  affordances or their adoption proof.
- RQ-3: Success requires a real-cycle dogfood and later reconstruction, not
  feature-level tests alone.
- RQ-4: `/askme` is a general facility for convenient semi-structured
  information acquisition from a human who detects the need; it is not tied to
  issue resolution or a particular process flow.
- RQ-5: Trigger-scoped practice injection (#198) and the design-integrity and
  vocabulary cluster (#200–203) are deferred. Their designs should wait for
  the forthcoming kan identity and data-model changes rather than freezing
  assumptions that are already moving.

## Open Questions

None.

## Out of Scope

- Declarable `/askme` prompts or another absent-means-default loader (#194).
- Trigger-scoped per-turn practice injection (#198).
- Design-criterion falsifiability, cross-design compatibility, a design-review
  atom, or a meta-evaluation atom (#200–203).
- Vocabulary packs and other transportable-vocabulary work.
- Persisting inferred process position, introducing a task database, or making
  streams synonymous with git worktrees.
- Automatically classifying interventions, retaining chat transcripts, or
  claiming an intervention log is exhaustive.
- Defining a day-owned identity or trust model ahead of kan's forthcoming
  identity and data-model changes.
- Making `/askme` itself a process atom or requiring it in any workflow.

***8<***
---
{
  "v": 3,
  "cid": "bafyreibdapr6ij6pzh4bpqwb2vdg2ykajncwtgfsd7qkdrhkf2mxisohyy",
  "sig": "dcfacfcc02bda7bd19a47a04c8692c648fadaa1750f1be43a6d248784283b827009d2a121fc654ac9ca3592052271b768a00a830be6af1f8e56d132b92be5076",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreihvbfqfwbx3dphoxrjcrwvb5rf2ujdmmdbvrxfqe34nih2pml3qhy"
  ],
  "rev": "223mt3enkbclb",
  "seq": 12,
  "of": 31,
  "text_len": 338,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9QlgWwb7G87rxSKNqh7EuqJGxgw1jcsCb41B9PYvcD5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQqnA6G4"
}
---

The v0.13 roadmap source of truth is the GitHub milestone plus day/kan claims, not a checked-in .design document. GitHub carries the release boundary and issue membership; kan carries the full detailed plan, telos, witness rationale, vocabulary, and bridge. Implementation artifacts that require versioning still belong in the repository.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa",
  "sig": "d12dc16e728e75b2d4f2b3d5a768fab38475d6d3a75471ac3322be76bf9eb20e6c1f32d5873ad71f14180d78d94e1890dbb3831688c7723fd4905693ee0ec715",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreihvbfqfwbx3dphoxrjcrwvb5rf2ujdmmdbvrxfqe34nih2pml3qhy"
  ],
  "rev": "223mt3lpdj32y",
  "seq": 13,
  "of": 31,
  "text_len": 338,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9QlgWwb7G87rxSKNqh7EuqJGxgw1jcsCb41B9PYvcD5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxql4Ou"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — The roadmap is pointed at the correct telos but is not yet executable or falsifiable: its canonical artifact is outside the declared design workflow, its release chronology cannot preserve the exact trialed commit, and six further evidence/provenance contracts remain incoherent.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigkpntub4x4good43jy3jh7vd76pk44zdmexzqpkh6ku2mqe43mya",
  "sig": "a94115add170bb5b3ecc1218690000f4ebbc5195e3bd506dad42660115bf5dc3535d8dfa326f9576f84282dc089e95ad3ea0d212832587ac7b063fdcf6f96044",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq2kcst",
  "seq": 14,
  "of": 31,
  "text_len": 503,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsCCKm"
}
---

F1 BLOCKING — The kan/milestone-only roadmap is not a first-class design-doc source. skills/design/SKILL.md:7 says the document feeds review; skills/adversarial-review/SKILL.md:43 says no file leaves no REQ/AC audit; src/record.rs:219 records from a path; day status currently reports design unmet. Standard review can infer an unrelated .design file. Either retain a checked-in design or teach status, validation, review retrieval, and witness probing to consume a kan Plan CID before implementation.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiakp2gwukfcsktmhpukqnyh6qipzxynmdf52cmv4fu4rx7ohsaaze",
  "sig": "cf341e5cb1686654fce4daeb71f40501d002c1e903975ff558191444fe90b77a2cb5b00a8d0b34ba9eabab2d925e699bf355ac881340719b42a3ac83e8ae2c6c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq2wfjg",
  "seq": 15,
  "of": 31,
  "text_len": 472,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsDi18"
}
---

F2 BLOCKING — The exact-trialed-commit release boundary conflicts with scripts/cut-release.sh:127 requiring the version first, :274 generating evidence after verification, and :368 creating a new release commit. AC-14 also asks post-trial evidence to be checked in. Define source equivalence or prepare and commit the final release candidate before trial, then tag without another commit; an external kan Result cannot also be required inside that same pre-trial commit.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifhesovr3lbuy2chk4btvcb26eilhtpg2cjaw25std2ygtw6u4hpe",
  "sig": "138cc2a90fafd7d3d8fa0d42801d8ea19f53494e6031bbf51260831987eb7e310d74a7373c9d8c3511ee676080938e466f8bb3b08a57fb68188308d386fffd59",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq3cmu3",
  "seq": 16,
  "of": 31,
  "text_len": 449,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsFErO"
}
---

F3 BLOCKING — The telos requires a fresh session to reconstruct what /askme acquired and changed, but REQ-7 forbids automatic recording and neither REQ-12 nor AC-14 requires an attributable summary carrier. Facts may be neither Decisions nor qualifying Interventions. Define an explicitly invoked Observation or scoped handoff contract with author, facts/decisions/unresolved separation, effect, citations, retrospective read, and removal control.
***8<***
---
{
  "v": 3,
  "cid": "bafyreies4nkbd4kwkxp5q6vl7r5llwnmc2wwm4vnwtmqts34f4yji6hu7e",
  "sig": "5dc32746fd432fce71eeba5822e4e33ccfa62a91748b5ef1bdca3eaef2ed47b5353ee1ca53c743c9a0f5519f80b49676632b891bb6c742614f76e2443a3ca17d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq3oy35",
  "seq": 17,
  "of": 31,
  "text_len": 441,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsGney"
}
---

F4 BLOCKING — AC-3/4 and AC-7/8/9 require behavioral execution of Markdown skills, but tests/plugin.rs and tests/agent_plugins.rs validate static packaging and tests/documented_invocations.rs only runs shell examples. No harness can supply sequential answers, observe adaptive one-question behavior, or prove a skill emitted no claim. Name a real execution/evidence mechanism or split static contracts from preregistered behavioral trials.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibhhtmv4ykqhjakcrswhxir7toyu5isrva7f36ttl5qsfq5bzn7yu",
  "sig": "456abd79840285fd8761a0c3ba52f43733453891e5aa72a032497d31ae7c5e0d7674bb293bd4ffbb0b5add10e261400e9ac73a472966341447c33ba1f1f66288",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq437pe",
  "seq": 18,
  "of": 31,
  "text_len": 369,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsIJY6"
}
---

F5 BLOCKING — AC-2 contradicts the supported kan range. tests/fixtures/kan-compat.tsv supports pre-role releases while tests/kan_conformance.rs:633 explicitly skips identity role add where unavailable. Decide and record a minimum-version break, update install/docs/matrix gates, or define a truthful compatible contract; disclosure does not satisfy actual authorship.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidli5ei4yijl5nrbv7am2bcwp7uzsd7idm5pxzvbge55aeekox5xi",
  "sig": "cc3a7fdc919c9a4d8a89fd2b25701d5155c7290dfb6d02e3110faf77d6c8670e78a10be790efb0b07f044570814028f9790c5383b8133701d260032f3a4d1643",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq4higv",
  "seq": 19,
  "of": 31,
  "text_len": 367,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsJrkt"
}
---

F6 BLOCKING — REQ-3 promises every thread and newest timestamp although src/kan_client.rs:184 makes recorded_at optional and :239/:330 permit withheld or unaccounted data whose subjects may be invisible. Define unknown timestamps and incomplete-view output, add withholding/unaccounted fixtures, and say every visible live thread unless completeness is established.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigc6jdr3lo7tf3jjdd7xnwi2grdwsor54xbimobnu7popmjz2jr5a",
  "sig": "3de6b8a68e94fb04d44b52f2dec3457764f1b8f50cb247409519a65512e8a4017738d602968e7ff38475b1eadfb5ea31b3f95efb42848de3e5cbfb85697c769a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq4turq",
  "seq": 20,
  "of": 31,
  "text_len": 338,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsLOqI"
}
---

F7 BLOCKING — REQ-8 asks who classified an intervention while AC-10 only preserves the recording identity. An agent may record at a humans direction, so signer and classifier differ; the telos later asks for the interventions author. Define classifier versus recorder, choose the authoritative field, and test the differing-actors case.
***8<***
---
{
  "v": 3,
  "cid": "bafyreickjxae3b7as6iuvwz5f4sdsl62thlvfzjnu3hiijnaofigl3vwoy",
  "sig": "2461f939b5311f800abe9e093471e23d8f9794aa9fad95f723affd02cc57577f69ad80fcd781522842e3d4e8a62e7e058c6e5d7e037cd08147c66effe441d3e0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3lq5ad7s",
  "seq": 21,
  "of": 31,
  "text_len": 420,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgE7EPJmOXLmlX/Rrcpe5rCs35X/PKzTjKQw/r5KjOlbhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQxsMyRI"
}
---

F8 BLOCKING — Issue and release-gate scope are incoherent. Issue #193 still specifies declarable schema/askme and automatic decision recording although the canonical plan defers both; scripts/cut-release.sh:177 verifies merged closure but not AC-13s required green checks. Record #193s split and define a fail-closed release-time check source. Also record whether #195 intervention kinds are fixed or project-additive.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidqxwlxr3zxjfwhnroazbpenfhvvca2hfihq4qxnigu4si7hqgt6i",
  "sig": "94f948ca6bc61f7fcc686de1854b24d3a2c434676d856fc9d31ced27962c86ca4a42b6f3bb68bc200a032693fc77e3dfa0f6181ea4b951dc07a38ee3b253f2ea",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt3lqct6nu",
  "seq": 22,
  "of": 31,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoNWQ2YzBjYzhmODcxMDllMjUxYjkxM2JjN2YyYWE4ZDNlN2ZjYmUzZml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkMbIySCw=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihmqxli3zvvpc3skydamumvmxe6oa2ja4bi6jdlofeyk63vsup3yy",
  "sig": "46f85b7cbbbe278a572e70ccf6a1c1f56e796c053dbdb0983ef5ef9923fe55e7195c4acfb30c8e8ecec6218a8b60e89424cc3e37cace779756b933990738be01",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreiakkusc2vsxtq5x62o4gemiee532vqlc5ayjy27sap36crtxqjdy4"
  ],
  "rev": "223mt3th2qyg7",
  "seq": 23,
  "of": 31,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgClUkLVZXnDt/adwxGIITu9VgsXQYTjX5AfvwozvBI8dmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5aC3kY"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 11 check(s), 0 failed, 2 warning(s), 0 unchecked, 0 open question(s) [doc 19515:73aa02a7a7430caf]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigaqmc43nyvs3j22fsrhay72pbcwf2f6soeakyoeghqxzvj4oengm",
  "sig": "d049e00c0d453ab377470165d9db8fdb1364dea6d138a239b9c4d218ec812dce5c0bb36e646ef6ef93dc8a52747743741d93dfd6b65cdb22e51da0046c674285",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreihmqxli3zvvpc3skydamumvmxe6oa2ja4bi6jdlofeyk63vsup3yy",
    "bafyreifk26vwrgfoizig3loe5vw7op4xoedmauwitg4r7pr22omc2hbz7i"
  ],
  "rev": "223mt3th35exf",
  "seq": 24,
  "of": 31,
  "text_len": 731,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiDshdaN5rV4tyVgYGUZVlyecDSQcCjyRrcUmFe3WVH7xtgqWCUAAXESIKrXq2iYrkZQba3E7W33P5dxBsBSyJm5H74605gtHDn6ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoNWQ2YzBjYzhmODcxMDllMjUxYjkxM2JjN2YyYWE4ZDNlN2ZjYmUzZml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkOWhGrPA=="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13` makes active work and meaningful human direction reconstructable from published claims without turning day into a task tracker or transcript. The release first adopts claim-addressed design artifacts and kan RFC1 authorship, then repairs handoff scope, ships an honest stream view, adds general `/askme` with explicit acquired-input recording, and records interventions without conflating authentic speech, repository admission, or consumer trust. A preregistered real-work reconstruction trial runs against the exact commit that is subsequently tagged and published. [validation: 11 check(s), 0 failed, 2 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreia2tkjtqncznfmvjoqsgv7vdpethd54gevxkedaxqbk2swgrwxjpq",
  "sig": "7aaeb6078bbba944975372d55e520bdbee8136402e516e722e133e454b21934c23c50bc3e7885904d465cd03c3975b34335057cbb8866cf3dcd6331d2de11856",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt3th3giux",
  "seq": 25,
  "of": 31,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2UmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljcywgY29ycmVjdGlvbiByb3VuZCAxbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDVkNmMwY2M4Zjg3MTA5ZTI1MWI5MTNiYzdmMmFhOGQzZTdmY2JlM2Zpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZDloWOvA="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicl33mhkexmhlz2ummotcsvb2eebkxlbou5p6anmvq2io6nibxv2y",
  "sig": "e27ae212eaadf9d7b8b121c3bea3c6f511acc7af7540ee8325f738ed9257e622706c92d595d413fbf25c8cd4af13050a807f51ce84292d5b605c9d9074b49114",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreigaqmc43nyvs3j22fsrhay72pbcwf2f6soeakyoeghqxzvj4oengm"
  ],
  "rev": "223mt3th42h25",
  "seq": 26,
  "of": 31,
  "text_len": 141,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwIMFzbcVltOtFlE4Mf08IrF0X0nEArDiGPC+ap44jTNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5aIDOI"
}
---

RQ-6: Stream listing promises visible live threads only and exposes unknown timestamps, withheld claims, and incomplete inventory explicitly.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiafu4qt52vsjbfuavzff5l4xdtf5m73d2r3phsjxczy67yflf2l7a",
  "sig": "18846b11804d587a5356c35b9f9b584a49773fb7ae81f1522940b1017eb046a16bd2e24742ae51d231e70790014f28e1f70834cbc280b300b3af89bf5107c10c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreigaqmc43nyvs3j22fsrhay72pbcwf2f6soeakyoeghqxzvj4oengm"
  ],
  "rev": "223mt3th4gqlq",
  "seq": 27,
  "of": 31,
  "text_len": 171,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwIMFzbcVltOtFlE4Mf08IrF0X0nEArDiGPC+ap44jTNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5aJlnI"
}
---

RQ-7: The claim signer is the intervention classifier. Human direction reported by an agent remains agent-authored unless separately authenticated human material is cited.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigwmu6vxxs24ue4666hiev3f5plfbhvzxcigve4no2hmx35orfzhe",
  "sig": "5146a0c86ccd372b2210edcc287e60677ee0b650445aa5386286321908bee99b273f60093a9e4098362f4b650baeab11aaacab5c52f7005804878b5710e6412c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreigaqmc43nyvs3j22fsrhay72pbcwf2f6soeakyoeghqxzvj4oengm"
  ],
  "rev": "223mt3th4syej",
  "seq": 28,
  "of": 31,
  "text_len": 214,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwIMFzbcVltOtFlE4Mf08IrF0X0nEArDiGPC+ap44jTNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ZDZjMGNjOGY4NzEwOWUyNTFiOTEzYmM3ZjJhYThkM2U3ZmNiZTNmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5aLHjf"
}
---

RQ-8: #193's declarable prompts remain #194; v0.13 ships fixed prompting and explicit acquired-input recording. Initial intervention kinds are fixed; project-additive vocabulary waits for the shared declared layer.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigvgfhlx6ev37o4o3syesmsb45qxalfy5k7z3vczxcm7hvf6zhb4a",
  "sig": "ed573def4871491d4310b4d2add4b06b9009fce4e317799b94ca5a72c10aa1fe1814f58ed5b908e35deb69c953fe08a6ae81504e9148a8d990ea09a9e672ef3d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreigaqmc43nyvs3j22fsrhay72pbcwf2f6soeakyoeghqxzvj4oengm",
    "bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa"
  ],
  "rev": "223mt3tiijx44",
  "seq": 29,
  "of": 31,
  "text_len": 19515,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiDAgwXNtxWW060WUTgx/TwisXRfScQCsOIY8L5qnjiNM9gqWCUAAXESIBOxDyZjly5pV/0a3KXuawrN+V/zys04ykMP6+SozpW4ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4KhZkNvbW1pdHgoYTZkNDBmY2UyMTQwMDBhYmU0MWExMDgwMzVjYzZmMjM3MDUyNGQzMqFmRmlsZUF0gngkLmRlc2lnbi92MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzLm1keChhNmQ0MGZjZTIxNDAwMGFiZTQxYTEwODAzNWNjNmYyMzcwNTI0ZDMyaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5c5/PG"
}
---

# Release: v0.13 workflow ergonomics, correction round 1

## Summary

`v0.13` makes active work and meaningful human direction reconstructable from
published claims without turning day into a task tracker or transcript. The
release first adopts claim-addressed design artifacts and kan RFC1 authorship,
then repairs handoff scope, ships an honest stream view, adds general `/askme`
with explicit acquired-input recording, and records interventions without
conflating authentic speech, repository admission, or consumer trust. A
preregistered real-work reconstruction trial runs against the exact commit that
is subsequently tagged and published.

This work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, and `telos/v1.0`. Its release target remains
`telos/v0.13-workflow-ergonomics`, requiring both the scoped `v0.13*`
`published-artifact` and `workflow-reconstruction-trial` witnesses.

## Requirements

- REQ-1: A v0.13 design is authoritative as a published kan `Plan` claim whose
  artifact address names an exact committed `.design/<slug>.md` mirror. The
  mirror supplies content to current validation, status, review, and git
  history; the Plan CID supplies identity, authority, and unambiguous review
  selection until kan's official claim-addressed content flow replaces the
  compatibility mirror.

- REQ-2: The v0.13 roadmap subject must publish through `kan publish` into the
  tracked `.claims/` tree. A reviewer given its Plan CID must recover the exact
  subject, artifact address, commit anchor, and byte-identical mirror without
  inferring the newest unrelated `.design` file.

- REQ-3: #196 must target kan RFC1 rather than legacy repository roles. Agent
  exploration and Plans are authentic speech of a disposable session-agent
  principal; human Decisions are first-hand only when signed by a verification
  method controlled by the human principal. Repository capability permits
  reach but never changes who spoke, and view trust remains a separate read
  result.

- REQ-4: RFC1-capable kan is a hard implementation prerequisite for #196. day
  must neither raise its compatibility floor merely to standardize the
  superseded `identity role add` surface nor ship disclosure as though it fixed
  actual authorship. Existing legacy claims remain readable under kan's
  compatibility projection.

- REQ-5: `/handoff` must record immutable coordinates for every time-relative
  verification: commit SHA for local suite results, explicit base and head for
  range censuses, and CI provider run identifier plus head SHA. `/wakeup`
  rechecks that scope; legacy unscoped measurements remain readable but are
  `UNCHECKABLE` rather than silently evaluated against the current tree (#152).

- REQ-6: `day stream list` must derive every visible live
  `agents/handoff/*` subject from one bulk kan read and report its name, live
  claim count, bounded preview, and newest timestamp when known (#204). If
  claims are withheld or unaccounted, or timestamps are absent, output must
  state that the inventory or recency is incomplete and must not say “every,”
  “newest,” or “stale” beyond what the view establishes.

- REQ-7: `skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md` must call the
  shared stream verb for list operations. Neither skill may retain an
  independent prose implementation of the fold or infer process position,
  worktree, or branch state for another stream.

- REQ-8: `/askme` is a general, non-atom driver affordance for adaptive
  one-question-at-a-time acquisition of semi-structured human input (#193).
  It establishes a topic, gathers available context, distinguishes supplied
  facts, decisions, and unresolved items, permits skip and stop, and writes no
  claim merely because a conversation occurred. Declarable prompts remain
  deferred to #194.

- REQ-9: After `/askme`, an explicit opt-in recording action may append an
  ordinary Observation carrying a `day-acquired-input` block. It must identify
  the work subject, topic, providing principal when authenticated or state that
  provenance is reported, recording author, facts, decisions, unresolved
  items, material effect, and cited basis. The skill summarizes and asks; it
  never treats silence or completion as consent to record.

- REQ-10: Intervention recording remains explicitly invoked and
  non-exhaustive (#195). The Observation author is the principal that actually
  classifies and records the event. An agent reporting human direction records
  agent-authored reported provenance unless separately authenticated human
  input can be cited; it must not certify a `classified_by` principal merely
  from prose. Reads expose the signer and reported or authenticated source
  without collapsing validity, repository admission, or view trust.

- REQ-11: Stream, `/askme`, acquired-input, and intervention semantics must be
  harness-agnostic. Deterministic tests cover parsing, packaging, prompt
  invariants, explicit write boundaries, log non-mutation, and serialization;
  preregistered real-harness protocols cover adaptive conversation behavior.
  Static keyword checks must not claim to test model behavior.

- REQ-12: Release preparation and publication must be separate operations.
  Preparation performs version and documentation changes, captures migration
  and block-corpus rows, runs verification, and commits the final candidate.
  After preregistration, all behavioral and reconstruction trials run against
  that exact SHA. Publication re-verifies immutable evidence and tags that SHA
  without modifying the tree.

- REQ-13: The release boundary consists of claim-addressed design support,
  RFC1 authorship (#196), immutable handoffs (#152), honest streams (#204),
  general `/askme` plus acquired-input recording (#193), and interventions
  (#195), each as a separately reviewed disposition. #193 must record that its
  original declarable/auto-recording proposal was split to #194 and the
  acquired-input convention.

- REQ-14: Release gating must fail closed on the exact required issue
  dispositions and exact required workflow runs for the prepared candidate
  SHA. A closed issue, a green run for another SHA, an unreadable GitHub
  response, or an unspecified check set cannot satisfy the gate.

- REQ-15: A preregistered `/askme` behavioral protocol must cover a decision
  request, factual request, unknown topic, skip, early stop, context-free repo,
  and explicit record/decline branches. Raw transcripts are trial evidence,
  not durable project state; an anchored Result reports the protocol outcome.

- REQ-16: The final real-work trial must select a visible stream, use `/askme`
  for a genuine need, explicitly record acquired input, record an intervention
  only if one genuinely occurs, and write a scoped handoff. A fresh session
  without the transcript must reconstruct the selected stream, acquired
  input's effect, intervention provenance, and verification scopes. Removing
  each required claim or coordinate must make its corresponding negative
  control fail.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) A published Plan fixture points to a committed
      `.design` mirror at an exact commit. Validation and review by CID recover
      byte-identical content; changing the mirror, commit, subject, or CID makes
      the check fail rather than selecting another design.

- [ ] AC-2: (REQ-1, REQ-2) `kan publish v0.13-workflow-ergonomics` produces a
      tracked `.claims/` representation containing the authoritative Plan, and
      a fresh clone can resolve its artifact address without local `.kan/`
      state.

- [ ] AC-3: (REQ-3, REQ-4) An RFC1 integration fixture records agent
      Observation/Plan claims under a session-agent principal and human
      Decision claims under a human-controlled verification method. It reports
      cryptographic validity, repository admission, and view inclusion
      separately and fails if either actor's speech is attributed to the other.

- [ ] AC-4: (REQ-3, REQ-4) Compatibility tests preserve legacy claim bytes and
      authorship while new writes use RFC1 principal and verification-method
      fields. If the required RFC1 write surface is unavailable, #196 and the
      v0.13 release gate remain blocked rather than falling back to roles or
      disclosure.

- [ ] AC-5: (REQ-5) A round-trip handoff fixture records suite, census, and CI
      scopes, advances and merges HEAD, then proves wakeup rechecks the original
      coordinates. A legacy unscoped fixture can never render `CONFIRMED`.

- [ ] AC-6: (REQ-6, REQ-7) Bulk-read fixtures cover live and superseded
      handoffs, retractions, unrelated subjects, missing timestamps, partially
      withheld claims, fully withheld subjects, and status/show unaccounted
      mismatches. Output returns visible rows plus explicit unknown/incomplete
      state and both skills invoke the shared verb.

- [ ] AC-7: (REQ-8) Static skill tests require topic establishment,
      one-question-at-a-time wording, fact/decision/unresolved separation,
      skip, stop, and explicit consent before recording. The skill remains
      useful with no issue, open kan subject, or active day atom.

- [ ] AC-8: (REQ-8, REQ-9) A scratch-log test runs every deterministic
      `/askme`-adjacent operation and proves the claim count is unchanged until
      the explicit acquired-input command is invoked. Decline and early stop
      append nothing.

- [ ] AC-9: (REQ-9) Acquired-input fixtures round-trip authenticated and
      reported providers, signer, facts, decisions, unresolved items, effect,
      subject, and citations. Empty effect, ambiguous provider provenance, or
      an attempt to encode a conversation transcript is rejected.

- [ ] AC-10: (REQ-10) Intervention fixtures cover same-actor classification,
      agent-authored reporting of human direction, and separately
      authenticated human input. Tests fail if reported provenance is rendered
      as first-hand human authorship or if an empty result is called proof that
      no intervention occurred.

- [ ] AC-11: (REQ-10, REQ-11) Hooks, prompts, sessions, and `/askme` cannot
      automatically emit acquired-input or intervention claims. Agent Skills
      and Claude-facing packaging expose the same semantics without creating a
      second durable store or Claude-only source of truth.

- [ ] AC-12: (REQ-11, REQ-15) Protocol fixtures and rubrics are committed
      before execution and name observable pass/fail conditions for every
      scenario. Static tests describe only contracts they execute; anchored
      behavioral Results cite raw trial artifacts and cannot be satisfied by a
      failed run or prose mentioning the pass marker.

- [ ] AC-13: (REQ-12) In a scratch release repo, preparation creates the sole
      candidate commit containing version, documentation, migration row, and
      block corpus. Trial evidence is recorded externally; publication tags
      that exact SHA with a clean tree and creates no commit. Reverting the
      split reproduces a tag/candidate mismatch.

- [ ] AC-14: (REQ-13, REQ-14) The release gate names the complete required
      issue and workflow set, verifies merged dispositions and successful runs
      at the candidate SHA, and fails on a missing issue, manual closure,
      absent merge, wrong SHA, skipped check, or unreadable API response.

- [ ] AC-15: (REQ-15) Real-harness trials demonstrate adaptive follow-ups,
      one-question pacing, unknown-topic narrowing, skip, stop, context-free
      usefulness, and explicit record/decline behavior. The rubric fails a
      transcript that merely contains required words without exhibiting the
      behavior.

- [ ] AC-16: (REQ-16) A fresh-session reconstruction Result names the exact
      candidate SHA, kan/RFC version, stream, acquired-input CID, optional but
      genuine intervention CID, scoped handoff claim, behavioral Result, and
      later wakeup evidence. Each preregistered removal control fails, and the
      Result plus `v0.13*` published artifact are both required for telos
      attainment.

## Architecture

### Published design with a committed compatibility mirror

The authoritative design is a kan `Plan` published into `.claims/`, not a file
chosen by modification time. During the transition to kan's official
claim-addressed content flow, the Plan carries an exact artifact address for
`.design/v0.13-workflow-ergonomics.md` at a commit. The committed mirror is
therefore not a competing source of truth: its bytes are content addressed by
the claim and exist so current `src/design.rs`, `src/record.rs`, `day status`,
`schema/witness/design-doc`, and `skills/adversarial-review/SKILL.md` can
operate. Review receives the Plan CID first and verifies the mirror before
reading requirements.

This correction round supersedes blocked verdict
`bafyreiatwehsmy4xfzuvp7i23ss642ykzx4v746kzu4muqyp5pskrtuvxa`. The old Plan
remains append-only history. The new Plan is recorded with `--file` and
published through `kan publish`; neither claim nor mirror is rewritten in
place after review.

### RFC1 identity rather than legacy roles

Kan RFC1 is the architecture boundary. `src/design.rs` and
`src/kan_client.rs` must consume its public principal, verification-method,
session-agent, governance, capability, admission, and view-result surfaces once
implemented. day does not interpret `.kan/roles`, mint principals, infer that
lineage grants authority, or turn a delegated agent into human speech.

If an interface allows the agent to submit a human-signed Decision, the human
verification method provides the proof. Otherwise the agent may authentically
report what it observed, but the record and UI label it reported provenance.
The same rule governs acquired input and interventions. Legacy records remain
visible through kan's compatibility projection without being re-signed.

### Honest scoped reads

Handoff coordinates remain prose claims whose required fields are mechanically
checked by the paired skills. Stream listing belongs in a reusable model near
the three-state read handling in `src/kan_client.rs`; rendering cannot outrun
`recorded_at: Option<_>`, withheld counts, or unaccounted-subject diagnostics.
The CLI under `src/cli/mod.rs` and both skill list paths consume that one model.

### Explicit acquired input and interventions

`skills/askme/SKILL.md` owns interaction policy and is deliberately not an
atom. A small explicit recording surface writes the `day-acquired-input` block
through kan's public CLI boundary. Parsing and rendering live with the existing
block and record modules in `src/blocks.rs` and `src/record.rs`; day stores no
conversation or private state.

Interventions use a separate ordinary Observation convention because their
meaning is different: acquired input says what was learned, while an
intervention says work materially changed or became possible. Both preserve
the actual signer. Authenticated provider material is cited; otherwise source
attribution is explicitly reported rather than cryptographically certified.

Initial intervention kinds are fixed semantic labels. Project-additive kinds
wait for the shared declared-preference and vocabulary-pack layer, avoiding a
new absent-means-default loader in v0.13.

### Two evidence planes for skills

`tests/plugin.rs`, `tests/agent_plugins.rs`, and
`tests/documented_invocations.rs` enforce deterministic structure, packaging,
commands, and non-mutation. They do not claim to run a model conversation.
Preregistered protocols exercise real Agent Skills consumers with raw
transcripts retained as CI/trial artifacts. Their kan Results carry anchored
markers and citations; the transcripts are not imported as project claims.

### Prepare, trial, publish

Refactor `scripts/cut-release.sh` into explicit preparation and publication
phases, with shared validation rather than duplicated shell. Preparation
performs every tree mutation—including Cargo/plugin versions, docs, migration
expectations, and block-corpus capture—and commits the sole candidate. The
candidate is pushed so GitHub workflows and real harness trials can name it.

Publication accepts the candidate SHA, requires a clean synchronized `main`,
re-reads the exact issue dispositions, required workflow conclusions,
behavioral Result, and reconstruction Result, and then records the release and
tags the candidate without a new commit. The append-only kan Results are
external evidence and therefore do not perturb the git identity they assess.

### Delivery order

1. Publish this correction-round Plan and committed mirror; cold-review both.
2. Wait for and verify kan RFC1's required public write/read surfaces.
3. Implement #196 against RFC1.
4. Implement #152 and its moving-HEAD round trip.
5. Implement #204 with incomplete-view semantics.
6. Update #193's split; implement `/askme` and acquired-input recording.
7. Implement #195 with authentic versus reported provenance.
8. Implement the prepare/publish split and fail-closed release manifest.
9. Preregister and run `/askme` behavioral trials.
10. Prepare the final candidate, preregister and run the real-work
    reconstruction, then publish that exact SHA.
11. Assess the published artifact and telos, then hand off exact coordinates.

Every implementation or correction round receives a fresh cold adversarial
review. BLOCK or REDIRECT findings are separately dispositioned before the next
phase; a previous review is never stretched to cover its fix.

## Resolved Questions

- RQ-1: Designs become published kan Plan claims. Until official kan
  claim-addressed content exists, each authoritative Plan points to a committed
  byte-verified `.design` compatibility mirror.
- RQ-2: Releases use prepare, trial, and publish phases; the published tag names
  the exact trialed candidate commit and trial Results remain external kan
  evidence.
- RQ-3: `/askme` records nothing automatically. An explicit
  `day-acquired-input` Observation carries its durable effect when requested.
- RQ-4: Deterministic tests cover executable contracts; preregistered real
  harness trials cover adaptive conversational behavior.
- RQ-5: #196 targets accepted kan RFC1 and waits for its implementation. day
  does not standardize the legacy role registry as its authorship model.
- RQ-6: Stream listing promises visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory explicitly.
- RQ-7: The claim signer is the intervention classifier. Human direction
  reported by an agent remains agent-authored unless separately authenticated
  human material is cited.
- RQ-8: #193's declarable prompts remain #194; v0.13 ships fixed prompting and
  explicit acquired-input recording. Initial intervention kinds are fixed;
  project-additive vocabulary waits for the shared declared layer.

## Open Questions

None.

## Out of Scope

- Implementing or modifying kan RFC1 inside the day repository.
- Treating legacy role names as principals, capabilities, or proof of human
  authorship.
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic recording of conversations, interventions, inferred process
  position, or human turns.
- Retaining raw transcripts as durable kan claims.
- Trigger-scoped practice injection (#198), design-integrity work (#200–203),
  vocabulary packs, and other v0.14 work.
- Making the non-author reconstruction trial a v0.13 release blocker.

***8<***
---
{
  "v": 3,
  "cid": "bafyreib6qwpuqn3erbltnyx6pl5a6hiegvnfjogo7lnoyab3bukrwog2be",
  "sig": "25d1ac64ef7ca8926fa19994b17629afb768bed723769f9cadc65abded2365826d300f410caf167d43c1eec27dfac36a970c569544d64eac0b0080608ad0a30a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt3tin7e47",
  "seq": 30,
  "of": 31,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChhNmQ0MGZjZTIxNDAwMGFiZTQxYTEwODAzNWNjNmYyMzcwNTI0ZDMyaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5dMqfX"
}
---
