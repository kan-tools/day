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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
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
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChhNmQ0MGZjZTIxNDAwMGFiZTQxYTEwODAzNWNjNmYyMzcwNTI0ZDMyaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ5dMqfX"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy",
  "sig": "fa523092949b36dd3e1857391b92db2836896877ce20cf2cd8956c7ef2bd9e6d7e199d5d93bb414576ee3efb7bbddf0ce8d48b1f97898a20329d7a5f81beacd3",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreigvgfhlx6ev37o4o3syesmsb45qxalfy5k7z3vczxcm7hvf6zhb4a"
  ],
  "rev": "223mt3uumkwgk",
  "seq": 31,
  "of": 125,
  "text_len": 354,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg1TFOu/iV393HblgkmSDzsLgWXHVfzuos3Ez56l9k4eBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ61KG6o"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Correction round 1 materially improves design identity, provenance, and RFC1 sequencing but remains unfalsifiable: its live trial witness is prefix-only, its behavioral trial precedes the final candidate, and publication, issue, workflow, read-surface, and admission contracts remain incomplete.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicfzosprhmptdyuiw5fo2fti7zp3c4edlispgo3cmcpl7wr363zsm",
  "sig": "62988c2a7be0ed3efe12696025e79391b6ee09120393eb2d4952ee4fc49270f703b875e978c22a42b64a128f574644cb1cb9af2ee68620fa108e377e22dfd05c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uv7nmd4",
  "seq": 32,
  "of": 125,
  "text_len": 453,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62Wci2"
}
---

ROUND-1 F1 BLOCKING — AC-12/15/16 require verified behavioral and reconstruction evidence, but schema/witness/workflow-reconstruction-trial accepts only a Result subject plus starts_with marker; src/probe.rs string-matches the prefix and cannot verify success, candidate SHA, scenarios, controls, or artifact retention. Define a concrete runner, immutable artifact schema/address, verifier, and witness probe that fails on false or incomplete bundles.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigfyfvn5duusxph6sr7nciiey5tlj6npz5sqdmt2vne6izneoxmfe",
  "sig": "24ea66536d179d4afef3bc62e258f4a3d76263b696330a3b5ef48674a240e2ec032c1f19a72e2db94a178022e4615b6a39d763f410d711a1eb8606085bb52cfb",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uva2mly",
  "seq": 33,
  "of": 125,
  "text_len": 309,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62YEnT"
}
---

ROUND-1 F2 BLOCKING — REQ-12 requires every behavioral and reconstruction trial against the final prepared candidate, but Delivery order step 9 runs askme trials before step 10 prepares that candidate. Prepare first, then run every release-significant trial at that exact SHA, then publish without mutation.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiae3yfgwdjh4buuj5yneogto5bgnp6dogppzamja4qtyh3573cwga",
  "sig": "0531314ac141aa2a88ad9d045cc1ffd4edb4d51fd5676b47688a436185ab6f522d14dc42dc06d6832943d7f56175f8268891bf34e6bece705d5ce34d42752060",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uvahnht",
  "seq": 34,
  "of": 125,
  "text_len": 320,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62Zs1O"
}
---

ROUND-1 F3 BLOCKING — REQ-16 and AC-16 make the intervention CID optional, while the live workflow-reconstruction witness rationale requires a genuine intervention and says a missing one leaves the witness absent. Continue real work until one occurs; never manufacture it, but do not permit a passing trial without it.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigemhpbqssrtg3s3qj5cmizro6rnsxj6kuu47dwqvvroduok3eigq",
  "sig": "6981357a8dd1c1d0c110833dfec73fa295a01bc515b317bdb27e199713bfde9f1be0b206d6ab42808dbe834f9c8b9e08fc3ea1bb0c17501742a1f344e2a1e134",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uvauurv",
  "seq": 35,
  "of": 125,
  "text_len": 436,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62bWqR"
}
---

ROUND-1 F4 BLOCKING — Plan-to-mirror bytes verify locally, but origin has no correction branch; milestone 15 still names the superseded Plan; and corrected RQ-1..RQ-5 reuse stable IDs already attached to different Decision texts, so day design record skipped them. Use new resolution IDs or explicit supersessions, update the milestone pointer, and make the published branch remotely retrievable before claiming fresh-clone authority.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicl5zkrenrzvcoj75nbrmzeou22j3fbgithvyzvdtzfedkt4yf5lm",
  "sig": "2fe67879c70c30c186d7b7427bd211dd4157b5c0c06107d5b574d7007ec8e1290871a10a842f08667c1d548352e116b97efffce23d905ba4429902008b143aab",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uvbbtca",
  "seq": 36,
  "of": 125,
  "text_len": 381,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62c+Sb"
}
---

ROUND-1 F5 BLOCKING — REQ-14 forbids an unspecified check set, but no authoritative release-manifest path/schema enumerates workflow names and required conclusions. Current issue bodies #196/#204/#193/#195 still contradict the correction Plan and milestone 15 carries 19 open issues plus the old Plan. Reconcile issue dispositions and define a fail-closed candidate-SHA manifest.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaacnqkti6ywnn6i2r3trbk24ewlikh35phueeqgrk66swqozfzqm",
  "sig": "72483e4e469e322fad88daf1a43f27fb1e89ec7dba45ca7d58cfa262497dbcfa4420c35c49685222fbf954a5c8a74826f1a576cd7d654028f9504c754c1a2796",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uvboxtv",
  "seq": 37,
  "of": 125,
  "text_len": 392,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62enbP"
}
---

ROUND-1 F6 BLOCKING — Stream honest-read design omits kan RFC1 published_read_error_count/published_read_errors, allowing damaged published claims to disappear while inventory looks complete. It also specifies only CLI and skills although src/mcp.rs and tests/mcp_server.rs require one-surface parity. Add published-read failures, visible-count wording, and MCP reuse of the same pure read.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifk37k2wbc3yq5poqlnzcfn67qzo5o6amdipybrq5khen5qhsdo2i",
  "sig": "3dd2aa6885cf17577d6da7ab95a4ed79b7ee5546a707dcd04ebcf5beb4efc00305a3061d796157e96bbc5f882d63bc18c71a18a3d9160474309181233414a8ad",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt3uvc436o",
  "seq": 38,
  "of": 125,
  "text_len": 396,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnGaOz/N1SGPaydzS9rPDFwq7HfJVhej9r2Uow/GpVbZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWQ62gQQt"
}
---

ROUND-1 F7 BLOCKING — RFC1 concepts are separated correctly, but AC-3 merely reports validity, admission, and view inclusion; it does not require both human and session-agent claims to be valid, admitted through an explicit capability path, and included under a named view. Specify capability provisioning, human signing, positive outcomes, and failure rendering at src/record.rs orchestration.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigsmx5y4dgrmixjpcvweytfb3azryhh3qflds32f3s46pqwksx7lm",
  "sig": "7d8f8516e6ae39aa65d31b020b1cf5ca770ecf4ca55cf8c09729ec24f04bfddf32f1c491e0c59b66a150bc0a2bb84ec129edfcd1bd182d84038cb834f1f39b52",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt3uvcjbrv",
  "seq": 39,
  "of": 125,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoN2E2NjZhYjQ5NzE5MTE3NzNjMjM5M2ZlOTVhZDUwYTkwZWM5YjcxYWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkOtoeekQ=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreig5ocuphvwkuzq6a2bfugw2tnffbt6yzj4d265hzxkn7ekd35feji",
  "sig": "b1e2862c5e765fdf6139b2057f3bf30ee6d8d1f22d8bb554987db39797c58daa2f45fff8acb3dd6ebd65b06d73613c463a18777455f450265cf0539672047233",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreihmqxli3zvvpc3skydamumvmxe6oa2ja4bi6jdlofeyk63vsup3yy"
  ],
  "rev": "223mt5ez6swyo",
  "seq": 40,
  "of": 125,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg7IXWjea1eLclYGBlGVZcnnA0kHAo8ka3FJhXt1lR+8ZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+THKc"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 13 check(s), 0 failed, 4 warning(s), 0 unchecked, 0 open question(s) [doc 25831:afe0e0fee509b289]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq",
  "sig": "1075f9ad2e42967d8f3d1c9d6c8db1b77d15a692e97c3ab9cc4f68ba8beb8b8e0afa990aab1c0aebe0bfde8b2e66d127cced3212e3ca89f863524f2b46eaee38",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreig5ocuphvwkuzq6a2bfugw2tnffbt6yzj4d265hzxkn7ekd35feji",
    "bafyreigaqmc43nyvs3j22fsrhay72pbcwf2f6soeakyoeghqxzvj4oengm"
  ],
  "rev": "223mt5ez7act2",
  "seq": 41,
  "of": 125,
  "text_len": 731,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiDdcKjz1sqmYeBoJaGtqbSlDP2Mp4PXunzdTfkUPfSkStgqWCUAAXESIMCDBc23FZbTrRZRODH9PCKxdF9JxAKw4hjwvmqeOI0zZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoN2E2NjZhYjQ5NzE5MTE3NzNjMjM5M2ZlOTVhZDUwYTkwZWM5YjcxYWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkavlMirA=="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13` makes active work and meaningful human direction reconstructable from published claims without turning day into a task tracker or transcript. The release first adopts claim-addressed design artifacts and kan RFC1 authorship, then repairs handoff scope, ships an honest stream view, adds general `/askme` with explicit acquired-input recording, and records interventions without conflating authentic speech, repository admission, or consumer trust. A preregistered real-work reconstruction trial runs against the exact commit that is subsequently tagged and published. [validation: 13 check(s), 0 failed, 4 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreice3ubtzfw2ynnzjgeosa66bjua6j4oqhunsafnf5npafazkjprje",
  "sig": "d691daae77b320f301bf61478cbabb2b03400def7ed04edfe029ad8c169f32943150233a185578e756779967d04dcaa35dc919c10e3594888c7ae00ee02a174b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt5ez7jld6",
  "seq": 42,
  "of": 125,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2UmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljcywgY29ycmVjdGlvbiByb3VuZCAxbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDdhNjY2YWI0OTcxOTExNzczYzIzOTNmZTk1YWQ1MGE5MGVjOWI3MWFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZGr5XxLY="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreielgp6i6oykke3nzadt6zcwa5jgq3fwaqasqxa7frrppxmpney3oq",
  "sig": "49501bdd6dfcce59462ba6539dfd58cd37c73dcd5fb1f354f636a31131a2f5752ade237391bec91c242f7716b29aa1cc89e1f1e5c5f43dc445a176233713a557",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5eza7aoz",
  "seq": 43,
  "of": 125,
  "text_len": 227,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+Ypoz"
}
---

RQ-9: Supersedes the earlier RQ-1 wording: designs become published kan Plan claims. Until official kan claim-addressed content exists, each authoritative Plan points to a committed byte-verified `.design` compatibility mirror.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifsycx5xvdzqqax5cvnng45s6mso5jcymqxqmttn5xhddrx6e6c5e",
  "sig": "cd4478e5eae8ce13e109e2a31dd0559a5cc89b53aa7a7b9157513d1da70a42c456c22eb26b7997fe9197184d9222396e6c17af8a60c4fa8f256a55a8af660cdb",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezamgpv",
  "seq": 44,
  "of": 125,
  "text_len": 199,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+aTJJ"
}
---

RQ-10: Supersedes the earlier RQ-2 wording: releases use prepare, trial, and publish phases; the published tag names the exact trialed candidate commit and trial Results remain external kan evidence.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig7zak7w4k4ihzvf7ig6io5go5swoqnr3xrin7lusw3zig4iea7vm",
  "sig": "ca76e43e820ddbddc98dbb907bdbc95a32b14e12e2cb314ad54c285565eef905103b0c9b4319d0258f729fe396cc1617a00f7eecbfff75678bd91060cab51d89",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezazmkf",
  "seq": 45,
  "of": 125,
  "text_len": 171,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+b8mX"
}
---

RQ-11: Supersedes the earlier RQ-3 wording: `/askme` records nothing automatically. An explicit `day-acquired-input` Observation carries its durable effect when requested.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieaw54k2cortdzupznm5vvxwjiyjpvyw6zlk57qipstydevhjsnhe",
  "sig": "d6619459e2c0d89f0ae1c53b935e1ca017a57a57e1de96c693f5054f392b237c26ecc874856ae7334230dec34cbf11529829118b79eda789a9e3ed1f4a96e101",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezbgsr5",
  "seq": 46,
  "of": 125,
  "text_len": 165,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+dmJ3"
}
---

RQ-12: Supersedes the earlier RQ-4 wording: deterministic tests cover executable contracts; preregistered real harness trials cover adaptive conversational behavior.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidbka7fpxx2bxl7wkyvivj7dhuem6rltj4mmtm4cntllctvtjbbra",
  "sig": "85f5315102a098c6191ceaa3cb148471605bd39335d9e16de0207ea7e23798a91d8404397868f61b36744b972b519c01cc6bb9af95af5a66d53cc80a8855f0e7",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezbtwkf",
  "seq": 47,
  "of": 125,
  "text_len": 183,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+fPGf"
}
---

RQ-13: Supersedes the earlier RQ-5 wording: #196 targets accepted kan RFC1 and waits for its implementation. day does not standardize the legacy role registry as its authorship model.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiasd5guk2i5lnxo3cb643mxrakalv6twhdor5lpb5tp56xkvvbeyq",
  "sig": "38fe615c7c7847e23e2a6a2c3b42607119db250a3f0de0f4a32dd54518bd16d618244b31c359c190bb32127ff060b5a0db94682bf789427f8db1ae36a5ea14c6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezcbax2",
  "seq": 48,
  "of": 125,
  "text_len": 188,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+g5s1"
}
---

RQ-14: Supersedes the correction-round RQ-6 wording: stream listing promises visible live threads only and exposes unknown timestamps, withheld claims, and incomplete inventory explicitly.
***8<***
---
{
  "v": 3,
  "cid": "bafyreift2bcwgg447icfqkbiv6yyftm5knvt2yjtvxrumykgecswxnobbe",
  "sig": "3f54e64c72a3dd29fda5e57a3d37c45b816541e256841d0e9bba3517c3eddab704ff971a8594a78cf17ac20b5d06acd37a8d31eb94bcea9342c5fe968ce9b727",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezcoo2s",
  "seq": 49,
  "of": 125,
  "text_len": 218,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+ik+l"
}
---

RQ-15: Supersedes the correction-round RQ-7 wording: the claim signer is the intervention classifier. Human direction reported by an agent remains agent-authored unless separately authenticated human material is cited.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie2xjaqwnpge3lvgpijm7aeyitjv34mtkbofqa2lnaybdbqyio5ee",
  "sig": "a7c05ffab15f715d48ac87b24f98a6d3ea8be7615e62c743148ed98683e7506f7a4b07feafbd9fb21468e33db453d148359c2fafa90156872b6693bbae5cebd4",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mt5ezd3ypk",
  "seq": 50,
  "of": 125,
  "text_len": 261,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMmm5E14T1vt47CaPv/A0BoYYvNwjt4lnw1VL5Bs855xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3YTY2NmFiNDk3MTkxMTc3M2MyMzkzZmU5NWFkNTBhOTBlYzliNzFhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRq+kPpE"
}
---

RQ-16: Supersedes the correction-round RQ-8 wording: #193's declarable prompts remain #194; v0.13 ships fixed prompting and explicit acquired-input recording. Initial intervention kinds are fixed; project-additive vocabulary waits for the shared declared layer.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiadnh5jip6qzt3z5kg4ja247csuzkjczcjricjsexx42xako7ho7q",
  "sig": "08b26b159fbdca2017f378e070638d9f3846293f57c0e2c6c57068cc3db5f2ac4c5151b0af5a1e4a44f101f2637a10ed83673d8aa7b8e9562bb9e38845698e4f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq",
    "bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy"
  ],
  "rev": "223mt5f24euvf",
  "seq": 51,
  "of": 125,
  "text_len": 25831,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiAyabkTXhPW+3jsJo+/8DQGhhi83CO3iWfDVUvkGzznnNgqWCUAAXESIJxmjs/zdUhj2snc0vazwxcKux3yVYXo/a9lKMPxqVW2ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4KhZkNvbW1pdHgoYTU1YjIwOThiMjdjNGRmM2Q2YjI3MGQ1MmIxYjM2Mzk3MmU3ZjJlMKFmRmlsZUF0gngkLmRlc2lnbi92MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzLm1keChhNTViMjA5OGIyN2M0ZGYzZDZiMjcwZDUyYjFiMzYzOTcyZTdmMmUwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRrAJWrv"
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
  “newest,” or “stale” beyond what the view establishes. Kan
  `published_read_error_count` and `published_read_errors` are completeness
  inputs alongside withheld and unaccounted state; a visible row count must be
  labelled visible rather than total whenever any narrowing is present.

- REQ-7: `skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md` must call the
  shared stream verb for list operations. Neither skill may retain an
  independent prose implementation of the fold or infer process position,
  worktree, or branch state for another stream. The same pure stream report
  must back CLI and MCP reads; shell availability cannot determine whether an
  agent can inspect streams.

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
  Static keyword checks must not claim to test model behavior. Raw trial
  bundles are committed to the remote `evidence/v0.13` branch; their immutable
  commit, manifest path, file digests, candidate SHA, and protocol digest are
  named by the signed Result.

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
  response, or an unspecified check set cannot satisfy the gate. The sole
  source is `.release/v0.13.json`, which enumerates issues 196, 152, 204, 193,
  and 195 and workflows `.github/workflows/ci.yml`,
  `.github/workflows/agent-plugins.yml`, `.github/workflows/kan-compat.yml`,
  `.github/workflows/migration-matrix.yml`,
  `.github/workflows/askme-behavioral-trial.yml`, and
  `.github/workflows/workflow-reconstruction-trial.yml`; every listed workflow
  must conclude success at the candidate SHA.

- REQ-15: A preregistered `/askme` behavioral protocol must cover a decision
  request, factual request, unknown topic, skip, early stop, context-free repo,
  and explicit record/decline branches. Raw transcripts are trial evidence,
  not durable claim content; they live in the immutable evidence commit. A
  structured `day-trial` Result reports the protocol, candidate, evidence
  address, scenario outcomes, and negative controls. `day trial verify` fetches
  and hashes the evidence, evaluates the manifest, and is the witness probe;
  no pass-marker string alone can satisfy it.

- REQ-16: The final real-work trial must select a visible stream, use `/askme`
  for a genuine need, explicitly record acquired input, continue genuine work
  until a qualifying intervention occurs, record that intervention, and write
  a scoped handoff. It must never manufacture an intervention, but it cannot
  pass without one. A fresh session
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
      separately. Both claims must be cryptographically valid, admitted through
      an explicit governance-rooted capability path covering their operation
      and subject, and included under a named view. The fixture fails if either
      actor's speech is attributed to the other, if lineage or role substitutes
      for capability, or if an authentic but unadmitted claim is rounded up to
      success.

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
      mismatches, nonzero `published_read_error_count`, and per-file published
      read errors. Output returns visible rows plus explicit unknown/incomplete
      state; both skills, CLI, and MCP invoke the same report function.

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
      second durable store or Claude-only source of truth. MCP and CLI stream
      results are byte-equivalent after transport framing.

- [ ] AC-12: (REQ-11, REQ-15) Protocol fixtures and rubrics are committed
      on the final candidate before execution and name observable pass/fail
      conditions for every scenario. Static tests describe only contracts they
      execute. `day trial verify` rejects a failed scenario, missing control,
      candidate or protocol mismatch, absent evidence commit, digest mismatch,
      malformed manifest, and a Result whose prose mentions a pass marker
      without a valid `day-trial` block.

- [ ] AC-13: (REQ-12) In a scratch release repo, preparation creates the sole
      candidate commit containing version, documentation, migration row, and
      block corpus. Trial evidence is recorded externally; publication tags
      that exact SHA with a clean tree and creates no commit. Reverting the
      split reproduces a tag/candidate mismatch.

- [ ] AC-14: (REQ-13, REQ-14) The release gate names the complete required
      issue and workflow set, verifies merged dispositions and successful runs
      at the candidate SHA, and fails on a missing issue, manual closure,
      absent merge, wrong SHA, skipped check, unlisted extra release dependency,
      malformed manifest, or unreadable API response. Mutation tests removing
      each issue and workflow from `.release/v0.13.json` make the gate fail.

- [ ] AC-15: (REQ-15) Real-harness trials demonstrate adaptive follow-ups,
      one-question pacing, unknown-topic narrowing, skip, stop, context-free
      usefulness, and explicit record/decline behavior. The rubric fails a
      transcript that merely contains required words without exhibiting the
      behavior. The evidence manifest names the real harness and model version,
      every transcript digest, scenario verdict, deviation, and rubric version;
      a fresh verifier reproduces the aggregate verdict from that bundle.

- [ ] AC-16: (REQ-16) A fresh-session reconstruction Result names the exact
      candidate SHA, kan/RFC version, stream, acquired-input CID, genuine
      intervention CID, scoped handoff claim, behavioral Result, and
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

Kan RFC1 is the architecture boundary. `src/record.rs` orchestrates per-claim
signing, `src/design.rs` supplies the parsed claim chain, and
`src/kan_client.rs` consumes
RFC1's public principal, verification-method,
session-agent, governance, capability, admission, and view-result surfaces once
implemented. day does not interpret `.kan/roles`, mint principals, infer that
lineage grants authority, or turn a delegated agent into human speech.

If an interface allows the agent to submit a human-signed Decision, the human
verification method provides the proof. Otherwise the agent may authentically
report what it observed, but the record and UI label it reported provenance.
The same rule governs acquired input and interventions. Legacy records remain
visible through kan's compatibility projection without being re-signed.

The integration fixture creates repository inception and governance, delegates
the minimum subject-and-operation capability to the disposable session agent,
and supplies the human verification method separately for each first-hand
Decision. It asserts positive `valid`, `admitted`, and named-view `included`
outcomes, then removes the capability and proves the same authentic agent claim
becomes explicitly unadmitted rather than disappearing or becoming invalid.

### Honest scoped reads

Handoff coordinates remain prose claims whose required fields are mechanically
checked by the paired skills. Stream listing belongs in a reusable model near
the three-state read handling in `src/kan_client.rs`; rendering cannot outrun
`recorded_at: Option<_>`, withheld counts, or unaccounted-subject diagnostics.
RFC1's published-read error count and per-file diagnostics participate in the
same completeness state. The CLI under `src/cli/mod.rs`, MCP under `src/mcp.rs`,
and both skill list paths consume that one model.

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
Preregistered protocols exercise real Agent Skills consumers. A runner writes
one manifest plus raw transcripts and command output, hashes every file, and
commits the bundle on `evidence/v0.13` without merging it into the candidate.
The branch remains remotely reachable; the Result names its immutable commit
and manifest path. `day trial verify` re-fetches that commit, refuses paths
outside it, verifies every digest and protocol/candidate coordinate, recomputes
scenario and control outcomes, and reports one of passed, failed, or
uncheckable. The telos witness invokes this verifier rather than matching Result
prose. Transcripts remain evidence, not imported claims or release-tree state.

The `day-trial` block is versioned and contains: protocol identifier and digest,
candidate SHA, harness/model versions, evidence repository and commit, manifest
path and digest, ordered scenario outcomes, ordered negative-control outcomes,
deviations, and aggregate verdict. Missing and unknown fields are refused by
the verifier. The signed Result authenticates this statement; the evidence
commit makes its referenced bytes independently retrievable.

### Prepare, trial, publish

Refactor `scripts/cut-release.sh` into explicit preparation and publication
phases, with shared validation rather than duplicated shell. Preparation
performs every tree mutation—including Cargo/plugin versions, docs, migration
expectations, and block-corpus capture—and commits the sole candidate. The
candidate is pushed so GitHub workflows and real harness trials can name it.

Publication accepts the candidate SHA, requires a clean synchronized `main`,
reads `.release/v0.13.json`, re-reads the exact issue dispositions and listed
workflow conclusions, behavioral Result, and reconstruction Result, and then
records the release and
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
9. Reconcile #196, #204, #193, and #195 with this Plan; update milestone 15 to
   this Plan CID; prepare, commit, and push the final candidate containing both
   trial protocols and `.release/v0.13.json`.
10. Run every listed CI and behavioral workflow at that exact candidate SHA.
11. Continue real work at the same candidate until a genuine intervention is
    available, then run and verify the reconstruction trial and all removal
    controls.
12. Publish the exact candidate SHA without a tree mutation.
13. Assess the published artifact and telos, then hand off exact coordinates.

Every implementation or correction round receives a fresh cold adversarial
review. BLOCK or REDIRECT findings are separately dispositioned before the next
phase; a previous review is never stretched to cover its fix.

## Resolved Questions

- RQ-1: The cycle ships the complete workflow-visibility set—stream view,
  general `/askme`, and intervention events—and the two evidence-correctness
  fixes they depend on. RQ-9 supersedes only its design-artifact storage model.
- RQ-2: #196 and #152 are release blockers and land before the new recording
  affordances or their adoption proof. RQ-10 adds the corrected release
  chronology without reversing this ordering.
- RQ-3: Success requires a real-cycle dogfood and later reconstruction, not
  feature-level tests alone. RQ-11 and RQ-12 specify its durable input and
  behavioral evidence.
- RQ-4: `/askme` is a general facility for convenient semi-structured human
  input, not issue resolution or a process atom. RQ-11 preserves this while
  adding explicit opt-in recording.
- RQ-5: Trigger-scoped practice injection and the design-integrity/vocabulary
  cluster remain deferred pending kan identity and data-model changes. RQ-13
  narrows the identity dependency to RFC1.
- RQ-6: Stream listing promises visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory. RQ-14 extends the
  same decision to published-read errors and MCP parity.
- RQ-7: The claim signer is the intervention classifier; human direction
  reported by an agent remains agent-authored unless authenticated material is
  cited. RQ-15 retains and clarifies this decision.
- RQ-8: #193's declarable prompts remain #194; v0.13 ships fixed prompting and
  explicit acquired-input recording with fixed initial intervention kinds.
  RQ-16 retains this split.
- RQ-9: Supersedes the earlier RQ-1 wording: designs become published kan Plan
  claims. Until official kan claim-addressed content exists, each authoritative
  Plan points to a committed
  byte-verified `.design` compatibility mirror.
- RQ-10: Supersedes the earlier RQ-2 wording: releases use prepare, trial, and
  publish phases; the published tag names
  the exact trialed candidate commit and trial Results remain external kan
  evidence.
- RQ-11: Supersedes the earlier RQ-3 wording: `/askme` records nothing
  automatically. An explicit
  `day-acquired-input` Observation carries its durable effect when requested.
- RQ-12: Supersedes the earlier RQ-4 wording: deterministic tests cover
  executable contracts; preregistered real
  harness trials cover adaptive conversational behavior.
- RQ-13: Supersedes the earlier RQ-5 wording: #196 targets accepted kan RFC1
  and waits for its implementation. day
  does not standardize the legacy role registry as its authorship model.
- RQ-14: Supersedes the correction-round RQ-6 wording: stream listing promises
  visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory explicitly.
- RQ-15: Supersedes the correction-round RQ-7 wording: the claim signer is the
  intervention classifier. Human direction
  reported by an agent remains agent-authored unless separately authenticated
  human material is cited.
- RQ-16: Supersedes the correction-round RQ-8 wording: #193's declarable prompts
  remain #194; v0.13 ships fixed prompting and
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
  "cid": "bafyreiderigpb42753xvlxbeqzvsymroexbqpgv6stmq66kk5p56bas724",
  "sig": "df35e07cadca9240d9e100a585b190dcd27136d4f48359fbd00e5567bab3460064be2e04eb76f4fd977dfd590f008844f7db1c329f450899e5dcf8ecef677f08",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt5f2egm6j",
  "seq": 52,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChhNTViMjA5OGIyN2M0ZGYzZDZiMjcwZDUyYjFiMzYzOTcyZTdmMmUwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWRrApkgg"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidp34ynfegg64bdow47fvxmhfbeypjjxjnwfxt3tpxn3i2jxt36je",
  "sig": "9742b08776f764ae956e3ea91ab6b63fa783667457a293467316d7f1ae606d50198f1b3a00bbf19d834b710d21e0256255b42bb56bbaa6b7af42bc1cee716d0e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreig5ocuphvwkuzq6a2bfugw2tnffbt6yzj4d265hzxkn7ekd35feji"
  ],
  "rev": "223mtc4hyg444",
  "seq": 53,
  "of": 125,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg3XCo89bKpmHgaCWhram0pQz9jKeD17p83U35FD30pEpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCb5gfF"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 15 check(s), 0 failed, 6 warning(s), 0 unchecked, 0 open question(s) [doc 35593:721ba05a0e622c66]
***8<***
---
{
  "v": 3,
  "cid": "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei",
  "sig": "576ff6ae25a1e2b4786c5b00ad9d81d46934aa2383ba216cc27a196966f392ef58432310c817cb472fc33845ec106b92d2b5986bfe7b6548885d31335605dd7d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreidp34ynfegg64bdow47fvxmhfbeypjjxjnwfxt3tpxn3i2jxt36je",
    "bafyreibsng4rgxqt235xr3bgr677anagqymlzxbdw6ewpq2vjpsbwphhtq"
  ],
  "rev": "223mtc4hz4pbb",
  "seq": 54,
  "of": 125,
  "text_len": 1038,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiBv3zDSkMb3Ajdbny1uw5Qkw9KbpbYt57m+7do0m89+SdgqWCUAAXESIDJpuRNeE9b7eOwmj7/wNAaGGLzcI7eJZ8NVS+QbPOecZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoYjUzM2JlODhiNTg0N2FlNTNiMTY5MTk5Y2NhMWRkMDRhNTI3OTUzMGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllAm/FUbQ=="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13` makes active work and meaningful human direction reconstructable from published claims without turning day into a task tracker or transcript. Its normative model is accepted RFC 1 Result `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`. The release first adopts claim-addressed design artifacts and RFC 1 authorship, then repairs handoff scope, ships an honest stream view, adds general `/askme` with explicit acquired-input recording, and records interventions without conflating authentic speech, repository admission, or consumer trust. Repository-owned procedures gather behavioral, reconstruction, and publication evidence at one candidate; a fresh profile-v1 assessment then certifies, does not certify, or cannot check the predeclared sufficient witness system. [validation: 15 check(s), 0 failed, 6 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreierynnx3t7r2uvhow5zon7b2n5hvakm3dvczuykgfq5hm5ezia4uq",
  "sig": "c77c969c72bdacad670b15ac56fffd3c0217146cb5a55e78648a61f54eb68ded15db2d21f274d9039dcb8d36295c8fa8d1cb20024b89fa608575c2755d5b44a6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtc4hzkoxr",
  "seq": 55,
  "of": 125,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2UmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljcywgY29ycmVjdGlvbiByb3VuZCAzbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KGI1MzNiZTg4YjU4NDdhZTUzYjE2OTE5OWNjYTFkZDA0YTUyNzk1MzBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZQJv4U0c="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaub3cshfacdx6z7u3eogrbr47zhs3x2w7e4jhtdj6cg3c7jxuvey",
  "sig": "a1dcca1c5d811d480347b6464c37869c903061b9804cf2b4f444614ba08aa5fc20fdd17d9784d058565b7ef240932517d69235c0cf012213c563824c78d8c687",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei"
  ],
  "rev": "223mtc4i35bhx",
  "seq": 56,
  "of": 125,
  "text_len": 330,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghv1FX0kNh6XSgbrrqy0uq+yUICzqEJ5K2uOax/ehoCJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCcEZ1D"
}
---

RQ-17: Accepted RFC 1 Result `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua` and source `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md` govern v0.13. Profile v1 is the implementation contract; the denotational target remains an explicit trajectory rather than a release prerequisite.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiblz46od3zjxftkteqtfwuuqu4s3gcmlng23khabmb4gesvwmlk6q",
  "sig": "fa0d69073399512a282a34029ef91b8b32cf5c4bd451c25ab9aedd57e804ad66584564a9d67dfe98fb56e1ed188cc822c719a1fbba734eff1a98e71201f19705",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei"
  ],
  "rev": "223mtc4i3tngm",
  "seq": 57,
  "of": 125,
  "text_len": 308,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghv1FX0kNh6XSgbrrqy0uq+yUICzqEJ5K2uOax/ehoCJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCcHM0a"
}
---

RQ-18: Supersedes the legacy two-witness release reading: v0.13 uses a predeclared v3 sufficient system joining publication, behavioral trial, and reconstruction trial evidence on one candidate. A stored trial Result is historical evidence; only fresh execution may derive a current `day-assessment` verdict.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicjvoi5ay2cmhojgxk6t4kr4q7q26ur7veti6twm7epbs4pxvi7yi",
  "sig": "1572303fe85260cb635d6ad384e9787f1f17c18ac7ec88bd3570f3db281bfe724ebcfa0ef2d10fa22728973ea68ea479ae9b9c67c60f27f5e1edec2634b97fab",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei"
  ],
  "rev": "223mtc4i4k3kb",
  "seq": 58,
  "of": 125,
  "text_len": 170,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghv1FX0kNh6XSgbrrqy0uq+yUICzqEJ5K2uOax/ehoCJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCcKAWR"
}
---

RQ-19: Release, trial, reconstruction, grading, and final assessment execution remain project-declared and repository-owned. v0.13 adds no release-specific day core verb.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie7ylwk2ver5ropwck5tyknndzchp5mcub5ql24bpwxbrs5u34rpi",
  "sig": "bdf04dcdbdff349d563c2710a16243d23128dbea0763c4c8860b18ec25f016a661258539d1d24b6c4ac70672fb70db2e1760afdfe3b854dc71223effd6f8faf8",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei"
  ],
  "rev": "223mtc4i5aqpw",
  "seq": 59,
  "of": 125,
  "text_len": 219,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghv1FX0kNh6XSgbrrqy0uq+yUICzqEJ5K2uOax/ehoCJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCcM1pB"
}
---

RQ-20: `.release/v0.13.json` is an instance, not its own authority. A typed `xtask` contract independently defines the exact issue, workflow, and post-publication artifact set, and the repository gate requires equality.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie4pncrroblyewoamingiwpnm3fhjdrforkxrfxvi77qtndajlj44",
  "sig": "ff2f80d7478a738b55d9f7b3a84ec4bd8eef6c10cacadec203f9a9df791a14f135fb171c03ed8354aa05b0af84518fed215f29adbe9153524b582a7c94a63b80",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei"
  ],
  "rev": "223mtc4i5xgtc",
  "seq": 60,
  "of": 125,
  "text_len": 189,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghv1FX0kNh6XSgbrrqy0uq+yUICzqEJ5K2uOax/ehoCJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCcPrKy"
}
---

RQ-21: RFC 1's seven deferred implementation areas are tracked by #227 through #233. Their closure, deferral, or narrowing cannot change accepted semantics without a superseding RFC or ADR.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidrpe7jsbkjrvy7gejjr2vqfk5gukwhrqzafyq2i4l7743z5sudme",
  "sig": "0133fd5935cdb63980216b6ae93022cf72df4194b3f38b38b838db36ac0f93b21c5977878eed7f04a41c2b8eeae7ec299b046bc7b2477f4bbdc93050e862ab61",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtc4ics73r",
  "seq": 61,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCcjBOP"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibor4fre333pqguax7wyckfzguf32xfeofexg3tfys4sdybeogowu",
  "sig": "b1d60241a4bf9a030c134464f469c4eaa68aa8465b834a45365f773a42b26e3755f1a62041b33d5fe621c95acb39f59b111e2fde045573cd5a63f229b69f0bdb",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreidp34ynfegg64bdow47fvxmhfbeypjjxjnwfxt3tpxn3i2jxt36je"
  ],
  "rev": "223mtc4j7u42r",
  "seq": 62,
  "of": 125,
  "text_len": 35592,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBv3zDSkMb3Ajdbny1uw5Qkw9KbpbYt57m+7do0m89+SWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOCoWZDb21taXR4KGI1MzNiZTg4YjU4NDdhZTUzYjE2OTE5OWNjYTFkZDA0YTUyNzk1MzChZkZpbGVBdIJ4JC5kZXNpZ24vdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljcy5tZHgoYjUzM2JlODhiNTg0N2FlNTNiMTY5MTk5Y2NhMWRkMDRhNTI3OTUzMGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllAnl0HiQ=="
}
---

# Release: v0.13 workflow ergonomics, correction round 3

## Summary

`v0.13` makes active work and meaningful human direction reconstructable from
published claims without turning day into a task tracker or transcript. Its
normative model is accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`.
The release first adopts claim-addressed design artifacts and RFC 1 authorship,
then repairs handoff scope, ships an honest stream view, adds general `/askme`
with explicit acquired-input recording, and records interventions without
conflating authentic speech, repository admission, or consumer trust.
Repository-owned procedures gather behavioral, reconstruction, and publication
evidence at one candidate; a fresh profile-v1 assessment then certifies, does
not certify, or cannot check the predeclared sufficient witness system.

This work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, and `telos/v1.0`. Its release target remains
`telos/v0.13-workflow-ergonomics`, but the current legacy flat witness list must
be replaced before trial execution by a profile-v1 sufficient declaration. The
declaration joins `published-artifact`, `askme-behavioral-trial`, and
`workflow-reconstruction-trial` on one exact `candidate` coordinate and names
the repository-owned assessment procedure that evaluates them.

This correction supersedes canonical Plan
`bafyreiadnh5jip6qzt3z5kg4ja247csuzkjczcjricjsexx42xako7ho7q` and responds to
BLOCK decision
`bafyreifg3vypawrxgngu3553yoivp66vxfsdwhjbxbfvc6qlebx2p5gu2u`. The superseded
Plan was correction round 2 and responded to its immediate predecessor BLOCK
`bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy`; its round-1
title and older lineage citation were stale.

## Requirements

- REQ-1: A v0.13 design is authoritative as a published kan `Plan` claim that
  cites the accepted RFC 1 Result and exact normative source above, and whose
  artifact address names an exact committed `.design/<slug>.md` mirror. The
  mirror supplies content to current validation, status, review, and git
  history; the Plan CID supplies identity, authority, and unambiguous review
  selection until kan's official claim-addressed content flow replaces the
  compatibility mirror.

- REQ-2: The v0.13 roadmap subject must publish through `kan publish` into the
  tracked `.claims/` tree. A reviewer given its Plan CID must recover the exact
  subject, artifact address, commit anchor, and byte-identical mirror without
  inferring the newest unrelated `.design` file.

- REQ-3: #196 must target accepted kan RFC 1 rather than legacy repository roles. Agent
  exploration and Plans are authentic speech of a disposable session-agent
  principal; human Decisions are first-hand only when signed by a verification
  method controlled by the human principal. Repository capability permits
  reach but never changes who spoke, and view trust remains a separate read
  result.

- REQ-4: RFC-1-capable kan is a hard implementation prerequisite for #196. day
  must neither raise its compatibility floor merely to standardize the
  superseded `identity role add` surface nor ship disclosure as though it fixed
  actual authorship. Existing legacy claims remain readable under kan's
  compatibility projection. Acceptance governs profile v1 only; the unresolved
  denotational target and deferred areas tracked by #227 through #233 are not
  v0.13 prerequisites and their disposition cannot revise RFC 1 without a
  superseding RFC or ADR.

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
  “newest,” or “stale” beyond what the view establishes. Kan
  `published_read_error_count` and `published_read_errors` are completeness
  inputs alongside withheld and unaccounted state; a visible row count must be
  labelled visible rather than total whenever any narrowing is present.
  Missing diagnostic fields are themselves incomplete input; deserialization
  must not default an absent count or list to a clean zero.

- REQ-7: `skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md` must call the
  shared stream verb for list operations. Neither skill may retain an
  independent prose implementation of the fold or infer process position,
  worktree, or branch state for another stream. The same pure stream report
  must back CLI and MCP reads; shell availability cannot determine whether an
  agent can inspect streams.

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
  Static keyword checks must not claim to test model behavior. Raw trial
  bundles are committed to the remote `evidence/v0.13` branch; their immutable
  commit, manifest path, file digests, candidate SHA, and protocol digest are
  named by attributable evidence claims. Stored trial claims are historical
  evidence, not witnesses, current verdicts, or telos certificates.

- REQ-12: Release preparation, trial execution, publication, and final
  assessment are project-declared procedures implemented in repository-owned
  `just`/`xtask` automation, not release-specific day verbs. Preparation
  performs version and documentation changes, captures migration and
  block-corpus rows, runs verification, and commits the final candidate. After
  preregistration, all behavioral and reconstruction trials run against that
  exact SHA. Publication re-verifies immutable evidence and tags that SHA
  without modifying the tree. Only a fresh post-publication execution of the
  predeclared assessment procedure may produce a current profile-v1 verdict.

- REQ-13: The release boundary consists of claim-addressed design support,
  RFC 1 authorship (#196), immutable handoffs (#152), honest streams (#204),
  general `/askme` plus acquired-input recording (#193), and interventions
  (#195), each as a separately reviewed disposition. #193 must record that its
  original declarable/auto-recording proposal was split to #194 and the
  acquired-input convention.

- REQ-14: Release gating must fail closed on the exact required issue
  dispositions and exact required workflow runs for the prepared candidate
  SHA. A closed issue, a green run for another SHA, an unreadable GitHub
  response, or an unspecified check set cannot satisfy the gate. A typed
  v0.13 contract in `xtask` is the independent canonical set; the candidate's
  `.release/v0.13.json` instance must equal it exactly rather than define its
  own completeness. The contract enumerates issues 196, 152, 204, 193, and 195
  and workflows `.github/workflows/ci.yml`,
  `.github/workflows/agent-plugins.yml`, `.github/workflows/kan-compat.yml`,
  `.github/workflows/migration-matrix.yml`,
  `.github/workflows/askme-behavioral-trial.yml`, and
  `.github/workflows/workflow-reconstruction-trial.yml`; every listed workflow
  must conclude success at the candidate SHA. The post-publication check also
  requires `.github/workflows/release.yml`, the intended tag target, installed
  crates.io package, GitHub Release, and release claim all to resolve to that
  same candidate.

- REQ-15: A preregistered `/askme` behavioral protocol must cover a decision
  request, factual request, unknown topic, skip, early stop, context-free repo,
  and explicit record/decline branches. Raw transcripts are trial evidence,
  not durable claim content; they live in the immutable evidence commit. A
  committed repository-owned grader or authenticated adjudication procedure
  derives each scenario outcome from named inputs, rubric version and digest,
  outputs, and per-check evidence coordinates. Mutation of a failing transcript
  while retaining a stored `passed` label must fail or become uncheckable; no
  pass-marker string or self-asserted scenario list can satisfy the component.

- REQ-16: The final real-work trial must select a visible stream, use `/askme`
  for a genuine need, explicitly record acquired input, continue genuine work
  until a qualifying intervention occurs, record that intervention, and write
  a scoped handoff. It must never manufacture an intervention, but it cannot
  pass without one. A fresh session
  without the transcript must reconstruct the selected stream, acquired
  input's effect, intervention provenance, and verification scopes. Removing
  each required claim or coordinate must make its corresponding negative
  control fail. Before either trial runs, the v0.13 telos is redeclared with a
  profile-v1 `day-telos` v3 sufficient system whose three components share the
  exact candidate coordinate and whose `procedure_spec` addresses a committed
  repository-owned assessment specification. After publication, a fresh run
  emits a Result with exactly one valid `day-assessment` block binding that
  declaration, procedure, evidence, per-component outcomes, and coordinate.
  Reading the stored Result later reports historical evidence and never a fresh
  verdict.

- REQ-17: The seven accepted RFC 1 deferred implementation areas remain
  explicit rollout records in #227 through #233. Each issue cites Result
  `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, exact source
  `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
  and the semantic obligation it implements.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) A published Plan fixture cites accepted RFC 1 Result
      `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, its exact
      normative source commit, and a committed `.design` mirror. Validation and
      review by CID recover byte-identical content; changing the RFC Result,
      source, mirror, commit, subject, or CID makes the check fail rather than
      selecting another design.

- [ ] AC-2: (REQ-1, REQ-2) `kan publish v0.13-workflow-ergonomics` produces a
      tracked `.claims/` representation containing the authoritative Plan, and
      a fresh clone can resolve its artifact address without local `.kan/`
      state.

- [ ] AC-3: (REQ-3, REQ-4) An RFC 1 integration fixture records agent
      Observation/Plan claims under a session-agent principal and human
      Decision claims under a human-controlled verification method. It reports
      cryptographic validity, repository admission, and view inclusion
      separately. Both claims must be cryptographically valid, admitted through
      an explicit governance-rooted capability path covering their operation
      and subject, and included under a named view. The fixture fails if either
      actor's speech is attributed to the other, if lineage or role substitutes
      for capability, or if an authentic but unadmitted claim is rounded up to
      success.

- [ ] AC-4: (REQ-3, REQ-4) Compatibility tests preserve legacy claim bytes and
      authorship while new writes use RFC 1 principal and verification-method
      fields. If the required RFC 1 write surface is unavailable, #196 and the
      v0.13 release gate remain blocked rather than falling back to roles or
      disclosure.

- [ ] AC-5: (REQ-5) A round-trip handoff fixture records suite, census, and CI
      scopes, advances and merges HEAD, then proves wakeup rechecks the original
      coordinates. A legacy unscoped fixture can never render `CONFIRMED`.

- [ ] AC-6: (REQ-6, REQ-7) Bulk-read fixtures cover live and superseded
      handoffs, retractions, unrelated subjects, missing timestamps, partially
      withheld claims, fully withheld subjects, and status/show unaccounted
      mismatches, nonzero `published_read_error_count`, and per-file published
      read errors. Output returns visible rows plus explicit unknown/incomplete
      state; omitting either published-read diagnostic field cannot deserialize
      as zero; both skills, CLI, and MCP invoke the same report function.

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
      second durable store or Claude-only source of truth. MCP and CLI stream
      results are byte-equivalent after transport framing. No shipped day CLI
      subcommand contains v0.13 release- or trial-specific execution policy.

- [ ] AC-12: (REQ-11, REQ-15) Protocol fixtures and rubrics are committed
      on the final candidate before execution and name observable pass/fail
      conditions for every scenario. Static tests describe only contracts they
      execute. The repository-owned grader rejects a failed scenario, missing
      control, candidate or protocol mismatch, absent evidence commit, digest
      mismatch, malformed manifest, and a stored pass label not derived from
      the addressed transcript and rubric. Replacing a failing transcript while
      retaining every asserted outcome is caught by a mutation test.

- [ ] AC-13: (REQ-12) In a scratch release repo, repository-owned `just`/`xtask`
      preparation creates the sole candidate commit containing version,
      documentation, migration row, and block corpus. Trial evidence is
      recorded externally; publication tags that exact SHA with a clean tree
      and creates no commit. Reverting the split reproduces a tag/candidate
      mismatch. A source scan and CLI inventory fail if the implementation adds
      a release-specific day verb.

- [ ] AC-14: (REQ-13, REQ-14) The release gate names the complete required
      issue and workflow set, verifies merged dispositions and successful runs
      at the candidate SHA, and fails on a missing issue, manual closure,
      absent merge, wrong SHA, skipped check, unlisted extra release dependency,
      malformed manifest, or unreadable API response. The manifest must equal
      the independently typed `xtask` contract. Mutation tests removing each
      issue and workflow from either side, or adding an unlisted obligation to
      either side, make the gate fail. A post-publication fixture also fails
      when the release workflow, tag, crate, GitHub Release, release claim, or
      either trial names a different candidate.

- [ ] AC-15: (REQ-15) Real-harness trials demonstrate adaptive follow-ups,
      one-question pacing, unknown-topic narrowing, skip, stop, context-free
      usefulness, and explicit record/decline behavior. The rubric fails a
      transcript that merely contains required words without exhibiting the
      behavior. The evidence manifest names the real harness and model version,
      every transcript digest, scenario verdict, deviation, grader identity,
      rubric version and digest, and per-check evidence coordinates; a fresh
      repository-owned execution reproduces the aggregate outcome from that
      bundle or reports it uncheckable.

- [ ] AC-16: (REQ-16) A profile-v1 fixture predeclares the three v0.13
      components and their shared `candidate` coordinate, then freshly executes
      the addressed repository procedure after publication. The resulting
      `day-assessment` certificate names the exact candidate SHA, procedure and
      declaration digests, tag/crate/GitHub Release/release-claim evidence,
      behavioral evidence, stream, acquired-input CID, genuine intervention
      CID, scoped handoff claim, and later wakeup evidence. Unequal candidate
      coordinates yield `not-certified`; missing or unreadable inputs yield
      `uncheckable`; reading the stored certificate cannot yield a current
      verdict. Each preregistered removal control fails.

- [ ] AC-17: (REQ-4, REQ-17) A tracker query finds exactly #227 through #233 as
      the seven RFC 1 deferred-area rollout records. Every issue body contains
      the accepted Result CID, exact source commit and path, and a distinct
      semantic obligation; closing or deferring any issue does not alter the
      accepted RFC bytes or status.

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

This correction round supersedes Plan
`bafyreiadnh5jip6qzt3z5kg4ja247csuzkjczcjricjsexx42xako7ho7q` and answers its
BLOCK
`bafyreifg3vypawrxgngu3553yoivp66vxfsdwhjbxbfvc6qlebx2p5gu2u`. The previous
correction's actual predecessor was
`bafyreie4m2hm743vjbr5vso42l3lhqyxbk5r34svqxup3l3ffdb7dkkvwy`; preserving all
three coordinates makes the append-only lineage explicit. The old Plan remains
history. The new Plan is recorded with `--file` and published through
`kan publish`; neither claim nor mirror is rewritten in place after review.

### Accepted RFC 1 identity rather than legacy roles

Accepted Result `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`
and exact source
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`
are the architecture boundary. `src/record.rs` orchestrates per-claim
signing, `src/design.rs` supplies the parsed claim chain, and
`src/kan_client.rs` consumes
RFC 1's public principal, verification-method,
session-agent, governance, capability, admission, and view-result surfaces once
implemented. day does not interpret `.kan/roles`, mint principals, infer that
lineage grants authority, or turn a delegated agent into human speech.

If an interface allows the agent to submit a human-signed Decision, the human
verification method provides the proof. Otherwise the agent may authentically
report what it observed, but the record and UI label it reported provenance.
The same rule governs acquired input and interventions. Legacy records remain
visible through kan's compatibility projection without being re-signed.

The integration fixture creates repository inception and governance, delegates
the minimum subject-and-operation capability to the disposable session agent,
and supplies the human verification method separately for each first-hand
Decision. It asserts positive `valid`, `admitted`, and named-view `included`
outcomes, then removes the capability and proves the same authentic agent claim
becomes explicitly unadmitted rather than disappearing or becoming invalid.

### Profile-v1 release assessment

The existing `telos/v0.13-workflow-ergonomics` declaration is a legacy flat
component report under RFC 1. Before trials execute, an explicit v3 sufficient
declaration replaces it with three project witness concepts:
`published-artifact`, `askme-behavioral-trial`, and
`workflow-reconstruction-trial`. Each component declares `candidate`; the
assembly requires exact equality across all three. A committed
`.release/v0.13-procedure.json` supplies the profile-v1 `procedure_spec` and
addresses a repository-owned `just`/`xtask` execution contract at the final
candidate. The declaration is therefore fixed before any component outcome is
available.

Trial manifests, transcripts, workflow runs, release artifacts, and kan claims
are evidence. The witness names are project concepts resolved by the declared
procedure; neither a probe nor a stored Result is the witness itself. After the
tag, crate, GitHub Release, and release claim exist, the repository procedure
runs freshly in one implicit local frame. It binds evidence CIDs and artifact
coordinates, derives each closed component outcome, checks the shared candidate
coordinate, and emits exactly one profile-v1 `day-assessment` block for an
explicit `kan result`. A later read may verify the certificate's bytes and
provenance but must render its verdict as historical rather than current.

This is release-specific automation in this repository, not a new `day trial`,
`day release`, or reconstruction subcommand. Day supplies generic declared
vocabulary, assessment, and certificate semantics; the project owns what its
release and trial procedures do.

### Honest scoped reads

Handoff coordinates remain prose claims whose required fields are mechanically
checked by the paired skills. Stream listing belongs in a reusable model near
the three-state read handling in `src/kan_client.rs`; rendering cannot outrun
`recorded_at: Option<_>`, withheld counts, or unaccounted-subject diagnostics.
RFC 1's published-read error count and per-file diagnostics participate in the
same completeness state. Presence is part of the contract: absent diagnostic
fields are unknown, never a clean default. The CLI under `src/cli/mod.rs`, MCP under `src/mcp.rs`,
and both skill list paths consume that one model.

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
Preregistered protocols exercise real Agent Skills consumers. A runner writes
one manifest plus raw transcripts and command output, hashes every file, and
commits the bundle on `evidence/v0.13` without merging it into the candidate.
The branch remains remotely reachable; attributable evidence claims name its
immutable commit and manifest path. The repository-owned grader re-fetches that
commit, refuses paths outside it, verifies every digest and
protocol/candidate coordinate, recomputes scenario and control outcomes from a
versioned rubric, and reports material, missing, or uncheckable component
evidence. Transcripts remain evidence, not imported claims or release-tree
state.

The evidence manifest contains the protocol identifier and digest, candidate
SHA, harness/model versions, evidence repository and commit, manifest path and
digest, grader identity, rubric version and digest, ordered scenario inputs and
derived outcomes, per-check evidence coordinates, negative controls, and
deviations. Missing and unknown fields are refused. The final
`day-assessment` certificate cites this evidence and derives its component
outcome; it does not trust an aggregate verdict copied from the manifest.

### Prepare, trial, publish, assess

Refactor `scripts/cut-release.sh` into repository-owned preparation and
publication phases exposed through `just` and typed `xtask`, with shared
validation rather than duplicated shell. Preparation
performs every tree mutation—including Cargo/plugin versions, docs, migration
expectations, and block-corpus capture—and commits the sole candidate. The
candidate is pushed so GitHub workflows and real harness trials can name it.

The typed v0.13 contract in `xtask` independently enumerates required issues,
workflows, and post-publication artifacts. `.release/v0.13.json` is an instance
that must equal that contract exactly; it cannot define its own completeness.
Pre-publication checks read the exact issue dispositions, candidate workflow
conclusions, and trial evidence. Publication accepts the candidate SHA,
requires a clean synchronized `main`, and tags the candidate without a new
commit. Post-publication checks bind `.github/workflows/release.yml`, the tag,
installed crates.io package, GitHub Release, release claim, behavioral
evidence, and reconstruction evidence to that candidate. Only then does the
fresh repository-owned assessment procedure derive the certificate recorded in
kan. Append-only kan evidence does not perturb the git identity it assesses.

### Delivery order

1. Publish this correction-round-3 Plan and committed mirror with the accepted
   RFC 1 Result and exact source coordinates; cold-review both.
2. Preserve #227 through #233 as the complete post-acceptance deferred-area
   rollout set. Verify kan RFC 1's required public write/read surfaces and
   implement the profile-v1 declaration, assessment, and certificate substrate
   required by this Plan without attempting the denotational target.
3. Implement #196 against accepted RFC 1.
4. Implement #152 and its moving-HEAD round trip.
5. Implement #204 with incomplete-view semantics.
6. Update #193's split; implement `/askme` and acquired-input recording.
7. Implement #195 with authentic versus reported provenance.
8. Implement repository-owned prepare/publish/assess automation, the typed
   v0.13 contract, and the fail-closed manifest instance.
9. Reconcile #196, #204, #193, and #195 with this Plan; update milestone 15 to
   this Plan CID; prepare, commit, and push the final candidate containing both
   trial protocols, `.release/v0.13-procedure.json`, and
   `.release/v0.13.json`. Redeclare the release telos as a v3 sufficient system
   addressed to that procedure before running trials.
10. Run every listed CI and behavioral workflow at that exact candidate SHA.
11. Continue real work at the same candidate until a genuine intervention is
    available, then run and verify the reconstruction trial and all removal
    controls.
12. Publish the exact candidate SHA without a tree mutation; verify the release
    workflow, tag, installed crate, GitHub Release, and release claim against
    the same candidate.
13. Freshly execute the addressed repository assessment procedure, record its
    profile-v1 `day-assessment` certificate, assess docs, and hand off exact
    coordinates.

Every implementation or correction round receives a fresh cold adversarial
review. BLOCK or REDIRECT findings are separately dispositioned before the next
phase; a previous review is never stretched to cover its fix.

## Resolved Questions

- RQ-1: The cycle ships the complete workflow-visibility set—stream view,
  general `/askme`, and intervention events—and the two evidence-correctness
  fixes they depend on. RQ-9 supersedes only its design-artifact storage model.
- RQ-2: #196 and #152 are release blockers and land before the new recording
  affordances or their adoption proof. RQ-10 adds the corrected release
  chronology without reversing this ordering.
- RQ-3: Success requires a real-cycle dogfood and later reconstruction, not
  feature-level tests alone. RQ-11 and RQ-12 specify its durable input and
  behavioral evidence.
- RQ-4: `/askme` is a general facility for convenient semi-structured human
  input, not issue resolution or a process atom. RQ-11 preserves this while
  adding explicit opt-in recording.
- RQ-5: Trigger-scoped practice injection and the design-integrity/vocabulary
  cluster remain deferred pending kan identity and data-model changes. RQ-13
  narrows the identity dependency to RFC 1.
- RQ-6: Stream listing promises visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory. RQ-14 extends the
  same decision to published-read errors and MCP parity.
- RQ-7: The claim signer is the intervention classifier; human direction
  reported by an agent remains agent-authored unless authenticated material is
  cited. RQ-15 retains and clarifies this decision.
- RQ-8: #193's declarable prompts remain #194; v0.13 ships fixed prompting and
  explicit acquired-input recording with fixed initial intervention kinds.
  RQ-16 retains this split.
- RQ-9: Supersedes the earlier RQ-1 wording: designs become published kan Plan
  claims. Until official kan claim-addressed content exists, each authoritative
  Plan points to a committed
  byte-verified `.design` compatibility mirror.
- RQ-10: Supersedes the earlier RQ-2 wording: releases use prepare, trial, and
  publish phases; the published tag names
  the exact trialed candidate commit and trial Results remain external kan
  evidence.
- RQ-11: Supersedes the earlier RQ-3 wording: `/askme` records nothing
  automatically. An explicit
  `day-acquired-input` Observation carries its durable effect when requested.
- RQ-12: Supersedes the earlier RQ-4 wording: deterministic tests cover
  executable contracts; preregistered real
  harness trials cover adaptive conversational behavior.
- RQ-13: Supersedes the earlier RQ-5 wording: #196 targets accepted kan RFC 1
  and waits for its implementation. day
  does not standardize the legacy role registry as its authorship model.
- RQ-14: Supersedes the correction-round RQ-6 wording: stream listing promises
  visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory explicitly.
- RQ-15: Supersedes the correction-round RQ-7 wording: the claim signer is the
  intervention classifier. Human direction
  reported by an agent remains agent-authored unless separately authenticated
  human material is cited.
- RQ-16: Supersedes the correction-round RQ-8 wording: #193's declarable prompts
  remain #194; v0.13 ships fixed prompting and
  explicit acquired-input recording. Initial intervention kinds are fixed;
  project-additive vocabulary waits for the shared declared layer.
- RQ-17: Accepted RFC 1 Result
  `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua` and source
  `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`
  govern v0.13. Profile v1 is the implementation contract; the denotational
  target remains an explicit trajectory rather than a release prerequisite.
- RQ-18: Supersedes the legacy two-witness release reading: v0.13 uses a
  predeclared v3 sufficient system joining publication, behavioral trial, and
  reconstruction trial evidence on one candidate. A stored trial Result is
  historical evidence; only fresh execution may derive a current
  `day-assessment` verdict.
- RQ-19: Release, trial, reconstruction, grading, and final assessment execution
  remain project-declared and repository-owned. v0.13 adds no release-specific
  day core verb.
- RQ-20: `.release/v0.13.json` is an instance, not its own authority. A typed
  `xtask` contract independently defines the exact issue, workflow, and
  post-publication artifact set, and the repository gate requires equality.
- RQ-21: RFC 1's seven deferred implementation areas are tracked by #227 through
  #233. Their closure, deferral, or narrowing cannot change accepted semantics
  without a superseding RFC or ADR.

## Open Questions

None.

## Out of Scope

- Implementing or modifying kan RFC 1 inside the day repository.
- Implementing RFC 1's full denotational target or the deferred work in
  #227 through #233 as part of v0.13.
- Treating legacy role names as principals, capabilities, or proof of human
  authorship.
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic recording of conversations, interventions, inferred process
  position, or human turns.
- Retaining raw transcripts as durable kan claims.
- Adding `day trial`, `day release`, or another release-specific core execution
  verb; the project owns those procedures through `just` and `xtask`.
- Trigger-scoped practice injection (#198), design-integrity work (#200–203),
  vocabulary packs, and other v0.14 work.
- Making the non-author reconstruction trial a v0.13 release blocker.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidn377lku3lr7w5b3klv75hmxj3qf6b3u36dmdbtepn4awco35d2q",
  "sig": "e27e3f5b23fdddd5f4490e2132d3227c3679bc1ac8fcf1df5b903b9d8cc5b1a60ecf01b305696417d54549f5f35375cbfa408488addc0de84e3fe6312e9b4c99",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtc4jdo3bw",
  "seq": 63,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChiNTMzYmU4OGI1ODQ3YWU1M2IxNjkxOTljY2ExZGQwNGE1Mjc5NTMwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUCemgSJ"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreib3t4pfqgs7x6sfsbb7fjzgykr7ilk75t2idl7dqdbmqn2aa7agie",
  "sig": "6b5e7702841dfce2d244df8ed0d585006fae3f483358bdecfb4be42f6b7121f92e1d6d321dafa1d90371dc96da4fe68b78648d29ca5c9192dde45c3a4517cd4e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreidp34ynfegg64bdow47fvxmhfbeypjjxjnwfxt3tpxn3i2jxt36je"
  ],
  "rev": "223mtc5bd726p",
  "seq": 64,
  "of": 125,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgb98w0pDG9wI3W58tbsOUJMPSm6W2Lee5vu3aNJvPfklmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkOTc0NmIxNGUwZjE1NDcxNTA2MzAwNTc0ZDJiOGZlYWYxOGMyYzMxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUDOkoAh"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 13 check(s), 0 failed, 4 warning(s), 0 unchecked, 0 open question(s) [doc 34915:cfe01a83befb467a]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigwitghalchjvnytvfkyj5cglvcpoo6cjivt2vnaeopwd6cw4klay",
  "sig": "afeb5129ab659d7a4ba698004a0878bb97e245d5ab606c89cbbddd3c4a04678467165ca3311f1f8c50567a75b9c6d361365427fd0adfa50eda64fd3fa4761945",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreib3t4pfqgs7x6sfsbb7fjzgykr7ilk75t2idl7dqdbmqn2aa7agie",
    "bafyreieg7vcv6sinq6s5fan25ovs2lvl5skcalhkccpevwxdtld7pinaei"
  ],
  "rev": "223mtc5bdvpyi",
  "seq": 65,
  "of": 125,
  "text_len": 1123,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiA7nx5YGl+/pFkEPypybCo/QtX+z0ga/jgMLIN0AHwGQdgqWCUAAXESIIb9RV9JDYel0oG666stLqvslCAs6hCeStrjmsf3oaAiZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoZDk3NDZiMTRlMGYxNTQ3MTUwNjMwMDU3NGQyYjhmZWFmMThjMmMzMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllAzp3XWQ=="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13` makes active work and meaningful human direction reconstructable from published claims without turning day into a task tracker or transcript. Its normative model is accepted RFC 1 Result `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`. The release first adopts claim-addressed design artifacts and RFC 1 authorship, then repairs handoff scope, ships an honest stream view, adds general `/askme` with explicit acquired-input recording, and records interventions without conflating authentic speech, repository admission, or consumer trust. Repository-owned procedures gather behavioral, reconstruction, and publication evidence at one candidate and fail closed when those coordinates disagree or cannot be checked. RFC 1 supplies the semantic guardrails for that work; v0.13 does not also build a generic profile-v1 declaration or certificate substrate. [validation: 13 check(s), 0 failed, 4 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaz7mwtfl6jo4lpaiqbemhgt7s7hx3dpubvobtaywqm37cpqjcmhe",
  "sig": "4f0d582cbb95e9ac0f45a4e7e3d06cee22a9210820d04c1469ac2091dc114a087c4f227de5fa2e02c93bce6f283fdf8f35b8f180f9347b0c3d21dc13c5c9d964",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtc5bedobn",
  "seq": 66,
  "of": 125,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2UmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljcywgY29ycmVjdGlvbiByb3VuZCA0bHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KGQ5NzQ2YjE0ZTBmMTU0NzE1MDYzMDA1NzRkMmI4ZmVhZjE4YzJjMzFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZQM6k0IQ="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreia4avqrqdmijmawkhejwrgzfducku7hnw2mkzozb6fqlni5y3xliy",
  "sig": "8549678f5359e3086227b35ee856698d43eac7374c9f1b42d63e509d0e6f13735f24fd0191d8d957062e622de6ef46a6a2e6c81c05601d99c454885525146283",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreigwitghalchjvnytvfkyj5cglvcpoo6cjivt2vnaeopwd6cw4klay"
  ],
  "rev": "223mtc5bfuzzz",
  "seq": 67,
  "of": 125,
  "text_len": 246,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg1kTMcCxHTVuJ1KrCeiMuonud4SUVnqrQEc+w/CtxSwZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkOTc0NmIxNGUwZjE1NDcxNTA2MzAwNTc0ZDJiOGZlYWYxOGMyYzMxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUDOvX+M"
}
---

RQ-22: The v1.0 non-author/third-party bar is the deliberate pass inward toward RFC 1 semantics. RFC 1 remains a constraint on every earlier release, but deeper generic realization and certification work does not displace the v0.13 critical path.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy",
  "sig": "6a7fb4524636f1539e3dc1e92283923e6f865b1f2ef2f312ddaa4c4ee36b4885363d4ee8a88f1f6651f6769afac7c34d117338353fec24e771fb69950ffea9fc",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreib3t4pfqgs7x6sfsbb7fjzgykr7ilk75t2idl7dqdbmqn2aa7agie"
  ],
  "rev": "223mtc5bloep3",
  "seq": 68,
  "of": 125,
  "text_len": 34914,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiA7nx5YGl+/pFkEPypybCo/QtX+z0ga/jgMLIN0AHwGQWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOCoWZDb21taXR4KGQ5NzQ2YjE0ZTBmMTU0NzE1MDYzMDA1NzRkMmI4ZmVhZjE4YzJjMzGhZkZpbGVBdIJ4JC5kZXNpZ24vdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljcy5tZHgoZDk3NDZiMTRlMGYxNTQ3MTUwNjMwMDU3NGQyYjhmZWFmMThjMmMzMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllAzxoqGg=="
}
---

# Release: v0.13 workflow ergonomics, correction round 4

## Summary

`v0.13` makes active work and meaningful human direction reconstructable from
published claims without turning day into a task tracker or transcript. Its
normative model is accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`.
The release first adopts claim-addressed design artifacts and RFC 1 authorship,
then repairs handoff scope, ships an honest stream view, adds general `/askme`
with explicit acquired-input recording, and records interventions without
conflating authentic speech, repository admission, or consumer trust.
Repository-owned procedures gather behavioral, reconstruction, and publication
evidence at one candidate and fail closed when those coordinates disagree or
cannot be checked. RFC 1 supplies the semantic guardrails for that work; v0.13
does not also build a generic profile-v1 declaration or certificate substrate.

This work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, and `telos/v1.0`. Its release target remains
`telos/v0.13-workflow-ergonomics`. Its project-owned release contract joins
publication, `/askme` behavioral trials, and workflow reconstruction on one
exact `candidate` coordinate without presenting that repository-specific gate
as a complete RFC 1 profile certificate.

This correction supersedes canonical correction-round-3 Plan
`bafyreibor4fre333pqguax7wyckfzguf32xfeofexg3tfys4sdybeogowu`. That Plan
correctly adopted RFC 1 but over-scoped v0.13 by making a generic profile-v1
declaration, assessment, and certificate substrate part of the release path.
This round preserves its provenance, evidence, and candidate-coordinate
constraints while deferring deliberate inward movement toward RFC 1 semantics
to the v1.0 bar.

## Requirements

- REQ-1: A v0.13 design is authoritative as a published kan `Plan` claim that
  cites the accepted RFC 1 Result and exact normative source above, and whose
  artifact address names an exact committed `.design/<slug>.md` mirror. The
  mirror supplies content to current validation, status, review, and git
  history; the Plan CID supplies identity, authority, and unambiguous review
  selection until kan's official claim-addressed content flow replaces the
  compatibility mirror.

- REQ-2: The v0.13 roadmap subject must publish through `kan publish` into the
  tracked `.claims/` tree. A reviewer given its Plan CID must recover the exact
  subject, artifact address, commit anchor, and byte-identical mirror without
  inferring the newest unrelated `.design` file.

- REQ-3: #196 must target accepted kan RFC 1 rather than legacy repository roles. Agent
  exploration and Plans are authentic speech of a disposable session-agent
  principal; human Decisions are first-hand only when signed by a verification
  method controlled by the human principal. Repository capability permits
  reach but never changes who spoke, and view trust remains a separate read
  result.

- REQ-4: RFC-1-capable kan is a hard implementation prerequisite for #196. day
  must neither raise its compatibility floor merely to standardize the
  superseded `identity role add` surface nor ship disclosure as though it fixed
  actual authorship. Existing legacy claims remain readable under kan's
  compatibility projection. Acceptance governs profile v1 only; the unresolved
  denotational target and deferred areas tracked by #227 through #233 are not
  v0.13 prerequisites and their disposition cannot revise RFC 1 without a
  superseding RFC or ADR.

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
  “newest,” or “stale” beyond what the view establishes. Kan
  `published_read_error_count` and `published_read_errors` are completeness
  inputs alongside withheld and unaccounted state; a visible row count must be
  labelled visible rather than total whenever any narrowing is present.
  Missing diagnostic fields are themselves incomplete input; deserialization
  must not default an absent count or list to a clean zero.

- REQ-7: `skills/handoff/SKILL.md` and `skills/wakeup/SKILL.md` must call the
  shared stream verb for list operations. Neither skill may retain an
  independent prose implementation of the fold or infer process position,
  worktree, or branch state for another stream. The same pure stream report
  must back CLI and MCP reads; shell availability cannot determine whether an
  agent can inspect streams.

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
  Static keyword checks must not claim to test model behavior. Raw trial
  bundles are committed to the remote `evidence/v0.13` branch; their immutable
  commit, manifest path, file digests, candidate SHA, and protocol digest are
  named by attributable evidence claims. Stored trial claims are historical
  evidence, not witnesses, current verdicts, or telos certificates.

- REQ-12: Release preparation, trial execution, publication, and final
  verification are project-declared procedures implemented in repository-owned
  `just`/`xtask` automation, not release-specific day verbs. Preparation
  performs version and documentation changes, captures migration and
  block-corpus rows, runs verification, and commits the final candidate. After
  preregistration, all behavioral and reconstruction trials run against that
  exact SHA. Publication re-verifies immutable evidence and tags that SHA
  without modifying the tree. Fresh post-publication verification reports the
  repository-specific release state and must not claim a generic profile-v1
  verdict.

- REQ-13: The release boundary consists of claim-addressed design support,
  RFC 1 authorship (#196), immutable handoffs (#152), honest streams (#204),
  general `/askme` plus acquired-input recording (#193), and interventions
  (#195), each as a separately reviewed disposition. #193 must record that its
  original declarable/auto-recording proposal was split to #194 and the
  acquired-input convention.

- REQ-14: Release gating must fail closed on the exact required issue
  dispositions and exact required workflow runs for the prepared candidate
  SHA. A closed issue, a green run for another SHA, an unreadable GitHub
  response, or an unspecified check set cannot satisfy the gate. A typed
  v0.13 contract in `xtask` is the independent canonical set; the candidate's
  `.release/v0.13.json` instance must equal it exactly rather than define its
  own completeness. The contract enumerates issues 196, 152, 204, 193, and 195
  and workflows `.github/workflows/ci.yml`,
  `.github/workflows/agent-plugins.yml`, `.github/workflows/kan-compat.yml`,
  `.github/workflows/migration-matrix.yml`,
  `.github/workflows/askme-behavioral-trial.yml`, and
  `.github/workflows/workflow-reconstruction-trial.yml`; every listed workflow
  must conclude success at the candidate SHA. The post-publication check also
  requires `.github/workflows/release.yml`, the intended tag target, installed
  crates.io package, GitHub Release, and release claim all to resolve to that
  same candidate.

- REQ-15: A preregistered `/askme` behavioral protocol must cover a decision
  request, factual request, unknown topic, skip, early stop, context-free repo,
  and explicit record/decline branches. Raw transcripts are trial evidence,
  not durable claim content; they live in the immutable evidence commit. A
  committed repository-owned grader or authenticated adjudication procedure
  derives each scenario outcome from named inputs, rubric version and digest,
  outputs, and per-check evidence coordinates. Mutation of a failing transcript
  while retaining a stored `passed` label must fail or become uncheckable; no
  pass-marker string or self-asserted scenario list can satisfy the component.

- REQ-16: The final real-work trial must select a visible stream, use `/askme`
  for a genuine need, explicitly record acquired input, continue genuine work
  until a qualifying intervention occurs, record that intervention, and write
  a scoped handoff. It must never manufacture an intervention, but it cannot
  pass without one. A fresh session
  without the transcript must reconstruct the selected stream, acquired
  input's effect, intervention provenance, and verification scopes. Removing
  each required claim or coordinate must make its corresponding negative
  control fail. The behavioral, reconstruction, and publication evidence must
  all name the same exact candidate. Unequal coordinates fail; missing or
  unreadable inputs are uncheckable. The repository records attributable
  evidence and its verification result through existing project surfaces,
  without requiring a v3 telos declaration or generic `day-assessment`
  certificate in v0.13.

- REQ-17: The seven accepted RFC 1 deferred implementation areas remain
  explicit rollout records in #227 through #233. Each issue cites Result
  `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, exact source
  `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
  and the semantic obligation it implements.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) A published Plan fixture cites accepted RFC 1 Result
      `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, its exact
      normative source commit, and a committed `.design` mirror. Validation and
      review by CID recover byte-identical content; changing the RFC Result,
      source, mirror, commit, subject, or CID makes the check fail rather than
      selecting another design.

- [ ] AC-2: (REQ-1, REQ-2) `kan publish v0.13-workflow-ergonomics` produces a
      tracked `.claims/` representation containing the authoritative Plan, and
      a fresh clone can resolve its artifact address without local `.kan/`
      state.

- [ ] AC-3: (REQ-3, REQ-4) An RFC 1 integration fixture records agent
      Observation/Plan claims under a session-agent principal and human
      Decision claims under a human-controlled verification method. It reports
      cryptographic validity, repository admission, and view inclusion
      separately. Both claims must be cryptographically valid, admitted through
      an explicit governance-rooted capability path covering their operation
      and subject, and included under a named view. The fixture fails if either
      actor's speech is attributed to the other, if lineage or role substitutes
      for capability, or if an authentic but unadmitted claim is rounded up to
      success.

- [ ] AC-4: (REQ-3, REQ-4) Compatibility tests preserve legacy claim bytes and
      authorship while new writes use RFC 1 principal and verification-method
      fields. If the required RFC 1 write surface is unavailable, #196 and the
      v0.13 release gate remain blocked rather than falling back to roles or
      disclosure.

- [ ] AC-5: (REQ-5) A round-trip handoff fixture records suite, census, and CI
      scopes, advances and merges HEAD, then proves wakeup rechecks the original
      coordinates. A legacy unscoped fixture can never render `CONFIRMED`.

- [ ] AC-6: (REQ-6, REQ-7) Bulk-read fixtures cover live and superseded
      handoffs, retractions, unrelated subjects, missing timestamps, partially
      withheld claims, fully withheld subjects, and status/show unaccounted
      mismatches, nonzero `published_read_error_count`, and per-file published
      read errors. Output returns visible rows plus explicit unknown/incomplete
      state; omitting either published-read diagnostic field cannot deserialize
      as zero; both skills, CLI, and MCP invoke the same report function.

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
      second durable store or Claude-only source of truth. MCP and CLI stream
      results are byte-equivalent after transport framing. No shipped day CLI
      subcommand contains v0.13 release- or trial-specific execution policy.

- [ ] AC-12: (REQ-11, REQ-15) Protocol fixtures and rubrics are committed
      on the final candidate before execution and name observable pass/fail
      conditions for every scenario. Static tests describe only contracts they
      execute. The repository-owned grader rejects a failed scenario, missing
      control, candidate or protocol mismatch, absent evidence commit, digest
      mismatch, malformed manifest, and a stored pass label not derived from
      the addressed transcript and rubric. Replacing a failing transcript while
      retaining every asserted outcome is caught by a mutation test.

- [ ] AC-13: (REQ-12) In a scratch release repo, repository-owned `just`/`xtask`
      preparation creates the sole candidate commit containing version,
      documentation, migration row, and block corpus. Trial evidence is
      recorded externally; publication tags that exact SHA with a clean tree
      and creates no commit. Reverting the split reproduces a tag/candidate
      mismatch. A source scan and CLI inventory fail if the implementation adds
      a release-specific day verb.

- [ ] AC-14: (REQ-13, REQ-14) The release gate names the complete required
      issue and workflow set, verifies merged dispositions and successful runs
      at the candidate SHA, and fails on a missing issue, manual closure,
      absent merge, wrong SHA, skipped check, unlisted extra release dependency,
      malformed manifest, or unreadable API response. The manifest must equal
      the independently typed `xtask` contract. Mutation tests removing each
      issue and workflow from either side, or adding an unlisted obligation to
      either side, make the gate fail. A post-publication fixture also fails
      when the release workflow, tag, crate, GitHub Release, release claim, or
      either trial names a different candidate.

- [ ] AC-15: (REQ-15) Real-harness trials demonstrate adaptive follow-ups,
      one-question pacing, unknown-topic narrowing, skip, stop, context-free
      usefulness, and explicit record/decline behavior. The rubric fails a
      transcript that merely contains required words without exhibiting the
      behavior. The evidence manifest names the real harness and model version,
      every transcript digest, scenario verdict, deviation, grader identity,
      rubric version and digest, and per-check evidence coordinates; a fresh
      repository-owned execution reproduces the aggregate outcome from that
      bundle or reports it uncheckable.

- [ ] AC-16: (REQ-16) A repository-owned fixture verifies that publication,
      behavioral, and reconstruction evidence name the exact same candidate
      SHA and binds that SHA to the tag, crate, GitHub Release, release claim,
      stream, acquired-input CID, genuine intervention CID, scoped handoff
      claim, and later wakeup evidence. Unequal candidate coordinates fail;
      missing or unreadable inputs are uncheckable; each preregistered removal
      control fails. The output is explicitly a v0.13 project verification
      result, not a generic profile-v1 certificate.

- [ ] AC-17: (REQ-4, REQ-17) A tracker query finds exactly #227 through #233 as
      the seven RFC 1 deferred-area rollout records. Every issue body contains
      the accepted Result CID, exact source commit and path, and a distinct
      semantic obligation; closing or deferring any issue does not alter the
      accepted RFC bytes or status.

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

This correction round supersedes Plan
`bafyreibor4fre333pqguax7wyckfzguf32xfeofexg3tfys4sdybeogowu`. That round
remains part of the append-only lineage, together with the earlier BLOCK and
correction coordinates it cites. The old Plan remains history. The new Plan is
recorded with `--file` and published through
`kan publish`; neither claim nor mirror is rewritten in place after review.

### Accepted RFC 1 identity rather than legacy roles

Accepted Result `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`
and exact source
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`
are the architecture boundary. `src/record.rs` orchestrates per-claim
signing, `src/design.rs` supplies the parsed claim chain, and
`src/kan_client.rs` consumes
RFC 1's public principal, verification-method,
session-agent, governance, capability, admission, and view-result surfaces once
implemented. day does not interpret `.kan/roles`, mint principals, infer that
lineage grants authority, or turn a delegated agent into human speech.

If an interface allows the agent to submit a human-signed Decision, the human
verification method provides the proof. Otherwise the agent may authentically
report what it observed, but the record and UI label it reported provenance.
The same rule governs acquired input and interventions. Legacy records remain
visible through kan's compatibility projection without being re-signed.

The integration fixture creates repository inception and governance, delegates
the minimum subject-and-operation capability to the disposable session agent,
and supplies the human verification method separately for each first-hand
Decision. It asserts positive `valid`, `admitted`, and named-view `included`
outcomes, then removes the capability and proves the same authentic agent claim
becomes explicitly unadmitted rather than disappearing or becoming invalid.

### RFC 1 guardrails, not a v0.13 substrate

RFC 1 constrains v0.13 wherever authorship, provenance, frame locality,
evidence, witnesses, admission, or view trust matter. In particular, authentic
speech is not repository authority, stored evidence is not a fresh witness,
and evidence gathered at different candidate coordinates cannot be silently
assembled into one release result.

Those distinctions do not require v0.13 to implement the generic realization,
v3 declaration, assessment, or certificate machinery described by the RFC's
profile model. The release instead uses a typed, repository-owned contract to
verify its exact issue, workflow, trial, reconstruction, and publication
obligations at one candidate. Its output is intentionally project-specific.
The v1.0 non-author/third-party bar is the deliberate pass inward toward the
generic RFC 1 semantics; #227 through #233 keep the accepted deferred areas
visible without pulling them onto the v0.13 critical path.

### Honest scoped reads

Handoff coordinates remain prose claims whose required fields are mechanically
checked by the paired skills. Stream listing belongs in a reusable model near
the three-state read handling in `src/kan_client.rs`; rendering cannot outrun
`recorded_at: Option<_>`, withheld counts, or unaccounted-subject diagnostics.
RFC 1's published-read error count and per-file diagnostics participate in the
same completeness state. Presence is part of the contract: absent diagnostic
fields are unknown, never a clean default. The CLI under `src/cli/mod.rs`, MCP under `src/mcp.rs`,
and both skill list paths consume that one model.

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
Preregistered protocols exercise real Agent Skills consumers. A runner writes
one manifest plus raw transcripts and command output, hashes every file, and
commits the bundle on `evidence/v0.13` without merging it into the candidate.
The branch remains remotely reachable; attributable evidence claims name its
immutable commit and manifest path. The repository-owned grader re-fetches that
commit, refuses paths outside it, verifies every digest and
protocol/candidate coordinate, recomputes scenario and control outcomes from a
versioned rubric, and reports material, missing, or uncheckable component
evidence. Transcripts remain evidence, not imported claims or release-tree
state.

The evidence manifest contains the protocol identifier and digest, candidate
SHA, harness/model versions, evidence repository and commit, manifest path and
digest, grader identity, rubric version and digest, ordered scenario inputs and
derived outcomes, per-check evidence coordinates, negative controls, and
deviations. Missing and unknown fields are refused. The repository-owned
verifier cites this evidence and derives its project outcome; it does not trust
an aggregate verdict copied from the manifest or present that outcome as a
generic RFC 1 certificate.

### Prepare, trial, publish

Refactor `scripts/cut-release.sh` into repository-owned preparation and
publication phases exposed through `just` and typed `xtask`, with shared
validation rather than duplicated shell. Preparation
performs every tree mutation—including Cargo/plugin versions, docs, migration
expectations, and block-corpus capture—and commits the sole candidate. The
candidate is pushed so GitHub workflows and real harness trials can name it.

The typed v0.13 contract in `xtask` independently enumerates required issues,
workflows, and post-publication artifacts. `.release/v0.13.json` is an instance
that must equal that contract exactly; it cannot define its own completeness.
Pre-publication checks read the exact issue dispositions, candidate workflow
conclusions, and trial evidence. Publication accepts the candidate SHA,
requires a clean synchronized `main`, and tags the candidate without a new
commit. Post-publication checks bind `.github/workflows/release.yml`, the tag,
installed crates.io package, GitHub Release, release claim, behavioral
evidence, and reconstruction evidence to that candidate. The fresh
repository-owned verifier records attributable project evidence through
existing surfaces. Append-only kan evidence does not perturb the git identity
it verifies.

### Delivery order

1. Publish this correction-round-4 Plan and committed mirror with the accepted
   RFC 1 Result and exact source coordinates; cold-review both.
2. Preserve #227 through #233 as the complete post-acceptance deferred-area
   rollout set. Verify only the RFC 1 public write/read surfaces required by
   #196; do not make generic profile-v1 machinery a v0.13 prerequisite.
3. Implement #196 against accepted RFC 1.
4. Implement #152 and its moving-HEAD round trip.
5. Implement #204 with incomplete-view semantics.
6. Update #193's split; implement `/askme` and acquired-input recording.
7. Implement #195 with authentic versus reported provenance.
8. Implement repository-owned prepare/publish/verify automation, the typed
   v0.13 contract, and the fail-closed manifest instance.
9. Reconcile #196, #204, #193, and #195 with this Plan; update milestone 15 to
   this Plan CID; prepare, commit, and push the final candidate containing both
   trial protocols and `.release/v0.13.json`.
10. Run every listed CI and behavioral workflow at that exact candidate SHA.
11. Continue real work at the same candidate until a genuine intervention is
    available, then run and verify the reconstruction trial and all removal
    controls.
12. Publish the exact candidate SHA without a tree mutation; verify the release
    workflow, tag, installed crate, GitHub Release, and release claim against
    the same candidate.
13. Freshly execute the repository verifier, record attributable project
    evidence through existing surfaces, assess docs and telos without claiming
    a generic profile-v1 certificate, and hand off exact coordinates.

Every implementation or correction round receives a fresh cold adversarial
review. BLOCK or REDIRECT findings are separately dispositioned before the next
phase; a previous review is never stretched to cover its fix.

## Resolved Questions

- RQ-1: The cycle ships the complete workflow-visibility set—stream view,
  general `/askme`, and intervention events—and the two evidence-correctness
  fixes they depend on. RQ-9 supersedes only its design-artifact storage model.
- RQ-2: #196 and #152 are release blockers and land before the new recording
  affordances or their adoption proof. RQ-10 adds the corrected release
  chronology without reversing this ordering.
- RQ-3: Success requires a real-cycle dogfood and later reconstruction, not
  feature-level tests alone. RQ-11 and RQ-12 specify its durable input and
  behavioral evidence.
- RQ-4: `/askme` is a general facility for convenient semi-structured human
  input, not issue resolution or a process atom. RQ-11 preserves this while
  adding explicit opt-in recording.
- RQ-5: Trigger-scoped practice injection and the design-integrity/vocabulary
  cluster remain deferred pending kan identity and data-model changes. RQ-13
  narrows the identity dependency to RFC 1.
- RQ-6: Stream listing promises visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory. RQ-14 extends the
  same decision to published-read errors and MCP parity.
- RQ-7: The claim signer is the intervention classifier; human direction
  reported by an agent remains agent-authored unless authenticated material is
  cited. RQ-15 retains and clarifies this decision.
- RQ-8: #193's declarable prompts remain #194; v0.13 ships fixed prompting and
  explicit acquired-input recording with fixed initial intervention kinds.
  RQ-16 retains this split.
- RQ-9: Supersedes the earlier RQ-1 wording: designs become published kan Plan
  claims. Until official kan claim-addressed content exists, each authoritative
  Plan points to a committed
  byte-verified `.design` compatibility mirror.
- RQ-10: Supersedes the earlier RQ-2 wording: releases use prepare, trial, and
  publish phases; the published tag names
  the exact trialed candidate commit and trial Results remain external kan
  evidence.
- RQ-11: Supersedes the earlier RQ-3 wording: `/askme` records nothing
  automatically. An explicit
  `day-acquired-input` Observation carries its durable effect when requested.
- RQ-12: Supersedes the earlier RQ-4 wording: deterministic tests cover
  executable contracts; preregistered real
  harness trials cover adaptive conversational behavior.
- RQ-13: Supersedes the earlier RQ-5 wording: #196 targets accepted kan RFC 1
  and waits for its implementation. day
  does not standardize the legacy role registry as its authorship model.
- RQ-14: Supersedes the correction-round RQ-6 wording: stream listing promises
  visible live threads only and exposes unknown
  timestamps, withheld claims, and incomplete inventory explicitly.
- RQ-15: Supersedes the correction-round RQ-7 wording: the claim signer is the
  intervention classifier. Human direction
  reported by an agent remains agent-authored unless separately authenticated
  human material is cited.
- RQ-16: Supersedes the correction-round RQ-8 wording: #193's declarable prompts
  remain #194; v0.13 ships fixed prompting and
  explicit acquired-input recording. Initial intervention kinds are fixed;
  project-additive vocabulary waits for the shared declared layer.
- RQ-17: Accepted RFC 1 Result
  `bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua` and source
  `35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`
  govern v0.13. Profile v1 supplies its semantic constraints, but generic
  realization is not a v0.13 implementation contract; the denotational target
  remains an explicit trajectory rather than a release prerequisite.
- RQ-18: Supersedes the correction-round-3 profile-v1 release reading: v0.13
  joins publication, behavioral trial, and reconstruction evidence on one
  candidate through a repository-specific gate. Generic v3 declaration and
  certificate reconciliation belongs to the deliberate v1.0 semantics pass;
  v0.13 evidence is not presented as a complete profile certificate.
- RQ-19: Release, trial, reconstruction, grading, and final verification
  remain project-declared and repository-owned. v0.13 adds no release-specific
  day core verb.
- RQ-20: `.release/v0.13.json` is an instance, not its own authority. A typed
  `xtask` contract independently defines the exact issue, workflow, and
  post-publication artifact set, and the repository gate requires equality.
- RQ-21: RFC 1's seven deferred implementation areas are tracked by #227 through
  #233. Their closure, deferral, or narrowing cannot change accepted semantics
  without a superseding RFC or ADR.
- RQ-22: The v1.0 non-author/third-party bar is the deliberate pass inward
  toward RFC 1 semantics. RFC 1 remains a constraint on every earlier release,
  but deeper generic realization and certification work does not displace the
  v0.13 critical path.

## Open Questions

None.

## Out of Scope

- Implementing or modifying kan RFC 1 inside the day repository.
- Implementing RFC 1's full denotational target or the deferred work in
  #227 through #233 as part of v0.13.
- Implementing a generic profile-v1 realization, v3 declaration, assessment,
  or certificate substrate as part of v0.13; that deliberate semantic pass
  begins at the v1.0 bar.
- Treating legacy role names as principals, capabilities, or proof of human
  authorship.
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic recording of conversations, interventions, inferred process
  position, or human turns.
- Retaining raw transcripts as durable kan claims.
- Adding `day trial`, `day release`, or another release-specific core execution
  verb; the project owns those procedures through `just` and `xtask`.
- Trigger-scoped practice injection (#198), design-integrity work (#200–203),
  vocabulary packs, and other v0.14 work.
- Making the non-author reconstruction trial a v0.13 release blocker.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihgt6ag75l352icy5devv5quhm2ly2mtmaun3nfsrlased5hq2cyq",
  "sig": "bfe7f688b6a7261677464c6646e598e1e40a3a774fa4a06dc3c980ab967a9a7f0cc10feefb4f56eb74e4dbdcc70d4ab5375c6f967fdd182bfa3cba6f2eed019e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtc5bp6gxt",
  "seq": 69,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkOTc0NmIxNGUwZjE1NDcxNTA2MzAwNTc0ZDJiOGZlYWYxOGMyYzMxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUDPUjNE"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa",
  "sig": "b48cd1faadc256d720d00d964085cfcbfedf35eeb6febf0ce4d67583104dd0b531c949e3a8c5e2400235e05ecdcfdfbd5b19f08088d547203e58c9e83019a1fe",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy"
  ],
  "rev": "223mtck7htduk",
  "seq": 70,
  "of": 125,
  "text_len": 237,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgL/4LV+0RpBms9kFti4sjk9KM1re3FOTEMutLsiLsq75mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQK3KbO"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Raw transcripts can be recorded as acquired-input facts; scoped-handoff and Plan-publication guarantees are nominal rather than executable; full v0.13 release evidence is absent.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidgeprmj73f6jn2okdrfc52l2r2ghfugekw2a7pdcuz3rumbf6wgm",
  "sig": "2bfc970bf41d538d7355c394791f795920daec9dedf22fd716df27c810e43b12486f0275b34f5d617b7d775e0fb854477323280ebba3b222c11cc79bb7accdf8",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa"
  ],
  "rev": "223mtcka2vyr4",
  "seq": 71,
  "of": 125,
  "text_len": 291,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgUH0HCSdGZ25aW6sjVRc0h0kn9rI77nG2lN1rs6Xy1ShmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQMDfp2"
}
---

BLOCKING: day acquired-input record rejects only an unsupported --transcript option, not transcript-shaped content. Hostile execution passed a dialogue in --fact and exited 0 while appending it; src/events.rs validates only non-emptiness, violating v0.13 AC-9 and the no-transcript boundary.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihwkjqz2ui6fkhirei5nfv2fzwmompnjovjxu7igwet7xwcce5yfe",
  "sig": "b60d429aa38d23de38e049c7a4b490ab98fe5ba4ccc9fcfee5cff8b332124bb649b6e29908d3ed0ca8f315a4c625bd30807ac2b36f170594e01f747edb16f729",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa"
  ],
  "rev": "223mtcka3pukl",
  "seq": 72,
  "of": 125,
  "text_len": 275,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgUH0HCSdGZ25aW6sjVRc0h0kn9rI77nG2lN1rs6Xy1ShmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQMGumZ"
}
---

MATERIAL: #152 has no moving-HEAD round-trip fixture or executable legacy-unscoped classification. tests/plugin.rs only checks that handoff/wakeup prose contains coordinate phrases, so AC-5 is not established and the test cannot fail when an agent ignores those instructions.
***8<***
---
{
  "v": 3,
  "cid": "bafyreif7hjhsdypv5mzv5ohphefc5xhpcpgww3eracv5jffipoaituq53e",
  "sig": "003f58693cd77a7c5a3751bfd344bce3c6506f3a46505eda8e091a02fef3e29365109936b8f5f14e74919cc998ada96c67be9ac83d4aaacb504ea75ff19abd74",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa"
  ],
  "rev": "223mtcka4ifuc",
  "seq": 73,
  "of": 125,
  "text_len": 395,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgUH0HCSdGZ25aW6sjVRc0h0kn9rI77nG2lN1rs6Xy1ShmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQMJy7U"
}
---

MATERIAL: authoritative Plan bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy has correct FileAt bytes and is published, but its sole citation edge targets validation Observation bafyreib3t4pfqgs7x6sfsbb7fjzgykr7ilk75t2idl7dqdbmqn2aa7agie, not accepted RFC 1 Result bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua; no v0.13 Plan fixture exercises the AC-1 mutation matrix.
***8<***
---
{
  "v": 3,
  "cid": "bafyreib4qg4rwekr5x3rcb723ufqoal5uzirl3fc3cafwkppbzkt6ftmjq",
  "sig": "9d7e8ab03c4cf4b27fe6541a70b1ac8b5e63f107fc588a2c57deeec1d6cca5c9036dbaa813bff32149c14b7ff7e058940fde23691e57201d9dc001db1519b552",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa"
  ],
  "rev": "223mtcka5bdsb",
  "seq": 74,
  "of": 125,
  "text_len": 325,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgUH0HCSdGZ25aW6sjVRc0h0kn9rI77nG2lN1rs6Xy1ShmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQMM6aT"
}
---

MATERIAL: /askme adaptive behavior, no-context usefulness, skip, stop, decline, and non-mutation are asserted by static keyword scans rather than executed real-harness protocols. No v0.13 evidence bundle, grader, candidate binding, or transcript mutation test exists, leaving REQ-11/15 and AC-7/8/12/15 unverifiable or unmet.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidwufva3l35eb6ybbossotry5vqldl6vknkiolwfsqsl6ztmv4jrq",
  "sig": "8b91bfa12170b5839012ea82fd457cbd2a607e9beb6649b5edf747656aa5c2826859fea77555ddebe0ced434348b515b25488c4f004362e89fa07fb24320b96d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa"
  ],
  "rev": "223mtcka5zhub",
  "seq": 75,
  "of": 125,
  "text_len": 297,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgUH0HCSdGZ25aW6sjVRc0h0kn9rI77nG2lN1rs6Xy1ShmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQMP7bR"
}
---

MATERIAL: intervention coverage is incomplete: production Source has only reported and authenticated-claim variants, and tests exercise two intervention kinds rather than the required same-actor, agent-reported-human, separately-authenticated matrix. The branch therefore does not establish AC-10.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiclbcpsnvj7m2ctzwhhjnf6tly5eapbf2yqx6pip6hl3y2hrxut7i",
  "sig": "05fe1f5b6c9a6e7248d55a1f901ff845c40c0f0494601fb63d5c797c28ddb17c056ac6279778a1620ea000c785893a9eb68d758e1454bc44368a81bb71c41b09",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa"
  ],
  "rev": "223mtcka6rkva",
  "seq": 76,
  "of": 125,
  "text_len": 390,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgUH0HCSdGZ25aW6sjVRc0h0kn9rI77nG2lN1rs6Xy1ShmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYWQ2YzRmMmViMjU0MzAzZjNhY2NmZjA3M2ExMGM1MzFjOTNiZmJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUQMS8Lu"
}
---

MATERIAL: the full correction-round-4 release contract is not present on this branch: #196 remains open and unimplemented, #152/#204/#193/#195 remain open, and repository-owned v0.13 prepare/trial/publish gates, typed contract, evidence grader, candidate-coordinate binding, reconstruction trial, and RFC deferred-area tracker gate are absent. REQ-3/4 and REQ-12 through REQ-17 cannot pass.
***8<***
---
{
  "v": 3,
  "cid": "bafyreih7dp5dtd2oiqvevr6p62ibv5rj2ugtqfpdh6k4ygfhksnniukv5y",
  "sig": "13091488221aea7aebda011f0c4c8f6e66ff93dc8a0fa169ac8eed0fa3752af76ebb81319851d766148437a1a9d4014808b1adc3ccc57500b1aed85f99297fe2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mtckaggw6r",
  "seq": 77,
  "of": 125,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoMGFkNmM0ZjJlYjI1NDMwM2YzYWNjZmYwNzNhMTBjNTMxYzkzYmZiYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllEDMZwJA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihb5v2qllcnzxckrdccp5eoitqwlnxnsrwlsd2fqhn26sjucorage",
  "sig": "56afd724aaa6312c29c562c8705f47d1967281471c0c622a091e74dfb9ccb7650b6838f19d7d4969a9ee88812d3cc565c35ce545fb4986a8bb25dd8287c44363",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua"
  ],
  "rev": "223mtclwsqnp4",
  "seq": 78,
  "of": 125,
  "text_len": 17602,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOCoWZDb21taXR4KGYyNTgwODBhMmRlZTU5ZTQ1ZmE5YzM3YmE2ZTY0MDhjYjJmMTA5MzehZkZpbGVBdIJ4JC5kZXNpZ24vdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljcy5tZHgoZjI1ODA4MGEyZGVlNTllNDVmYTljMzdiYTZlNjQwOGNiMmYxMDkzN2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllEeYtOHw=="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, harness version, and file digests. The repository grader recomputes
  each outcome from addressed evidence; changing a transcript while updating
  its digest must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The grader binds
  their CIDs and suite/census/CI coordinates to one candidate, checks them
  against bulk kan and wakeup evidence, and itself derives every removal and
  wrong-candidate control.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction consumes an explicitly addressed evidence commit. Missing,
  malformed, unreadable, wrong-SHA, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, or transcript content in durable claims.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate. All eleven registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work bundle. Both
use graders in `xtask`; neither trusts a stored `passed` string.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibdlgk5nd43wrxlsu663avscedxxrvvn7zqpzsa467kv3oqm5d6hi",
  "sig": "8ecdfe26eac9548dd97d53f861e0e4769ed1800222abbe65dbddc8bc714012a9107da0827a07db24d39228ad5b2e8d6c3b63bd2aa55f031efde406ae1aeb9026",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtclwwxw37",
  "seq": 79,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChmMjU4MDgwYTJkZWU1OWU0NWZhOWMzN2JhNmU2NDA4Y2IyZjEwOTM3aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUR5zu+0"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifapex5udoki4mzdd2q37qjaudjljj7cw6smnjj2qum6cbrojll5q",
  "sig": "dd985ca4bdeb05f7386907161fcc255665f90bc6fa8f2aa38eacb363daccaaab17d3cbd893e001024955dbb59ca1bbf29d5c56ab41f37b2a887ea14bfd045349",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreihb5v2qllcnzxckrdccp5eoitqwlnxnsrwlsd2fqhn26sjucorage"
  ],
  "rev": "223mtcm2nvnta",
  "seq": 80,
  "of": 125,
  "text_len": 17603,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboNgqWCUAAXESIOHtdQWsTc3EqIxCf0jkThZbbtlGy5D0WB269JNBOiAxZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4KhZkNvbW1pdHgoZjI1ODA4MGEyZGVlNTllNDVmYTljMzdiYTZlNjQwOGNiMmYxMDkzN6FmRmlsZUF0gngkLmRlc2lnbi92MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzLm1keChmMjU4MDgwYTJkZWU1OWU0NWZhOWMzN2JhNmU2NDA4Y2IyZjEwOTM3aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUSBPc6l"
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, harness version, and file digests. The repository grader recomputes
  each outcome from addressed evidence; changing a transcript while updating
  its digest must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The grader binds
  their CIDs and suite/census/CI coordinates to one candidate, checks them
  against bulk kan and wakeup evidence, and itself derives every removal and
  wrong-candidate control.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction consumes an explicitly addressed evidence commit. Missing,
  malformed, unreadable, wrong-SHA, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, or transcript content in durable claims.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate. All eleven registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work bundle. Both
use graders in `xtask`; neither trusts a stored `passed` string.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreih32xbifqogsjn2eqvgm4mopw4ejoity2bifo2kamphwi2hd4azlu",
  "sig": "d356ee1d2f583722f1f8c955a89a0a28ad6cbd4ac6aff0cc0a27ad1aa44809657541002aa3dea6d6ea20bd0f1d26ee39d901f97b98ee068e02c7f3e9fa1ab36f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcm2sushg",
  "seq": 81,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChmMjU4MDgwYTJkZWU1OWU0NWZhOWMzN2JhNmU2NDA4Y2IyZjEwOTM3aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUSBjWE1"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifpbz6x3kaozus5ayuftdwrewrkfcdh4x6j7ecsfvu55w665o7ykm",
  "sig": "b84b4123337ba6515c8abb99888d614a53bd4d094a6783729ababf309d84b2c91789cc3b21a5c160053c93ecdf56a55df08d417b4e3ec75e28d5029c6238a5b1",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreifapex5udoki4mzdd2q37qjaudjljj7cw6smnjj2qum6cbrojll5q"
  ],
  "rev": "223mtcniq52ys",
  "seq": 82,
  "of": 125,
  "text_len": 17603,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboNgqWCUAAXESIKB5L9oNykcZkY9Q3+CQUGlaU/Fb0mNSnUKM8IMXJWvsZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4KhZkNvbW1pdHgoZjMwOTQzMTBmMjNiMzUxNGFmMmMzYWJkNzQzZDlkYTFkZDE3N2U0N6FmRmlsZUF0gngkLmRlc2lnbi92MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzLm1keChmMzA5NDMxMGYyM2IzNTE0YWYyYzNhYmQ3NDNkOWRhMWRkMTc3ZTQ3aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUTdYYIT"
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, harness version, and file digests. The repository grader recomputes
  each outcome from addressed evidence; changing a transcript while updating
  its digest must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The grader binds
  their CIDs and suite/census/CI coordinates to one candidate, checks them
  against bulk kan and wakeup evidence, and itself derives every removal and
  wrong-candidate control.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction consumes an explicitly addressed evidence commit. Missing,
  malformed, unreadable, wrong-SHA, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, or transcript content in durable claims.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate. All eleven registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work bundle. Both
use graders in `xtask`; neither trusts a stored `passed` string.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreie4apmz7vjvlnhlr45eyoicgwag352gpuzdir4s3d37yix7ywboqq",
  "sig": "18a83f0683643afcc9e6e6041a4f1173ef95facdb98ef4552486e18283cdeebc033bdd473a0115db24bd0b99841bff4af7ac67469cee832467744728cea6af4d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcniqxiyl",
  "seq": 83,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChmMzA5NDMxMGYyM2IzNTE0YWYyYzNhYmQ3NDNkOWRhMWRkMTc3ZTQ3aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUTdbrtQ"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihzy65oajegvv5gd6tfrwgxh4kpyaifstqp66m4jqr3z62a7bx2hy",
  "sig": "44abcd72b140b4e0af47a5e91321fdb76b095d449d04cf17defc510cd546d9194fdf55c6d055cbe3cb45667e04755772a7b5e0b196a3516700e9da0e1f3e12ad",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreifpbz6x3kaozus5ayuftdwrewrkfcdh4x6j7ecsfvu55w665o7ykm"
  ],
  "rev": "223mtcoapzvt7",
  "seq": 84,
  "of": 125,
  "text_len": 240,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrw59fagOzSXQYoWY7RJaKiiGfl/J+QUi1p3tve67+FNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0NTdmZTQyZTcwYmE0Yzg4MzIwZDYxMzQ5NjQ5YTM1ODVmMzlkOGVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUUNX+6w"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Cold hostile probes certified fabricated reconstruction tokens and admitted named-speaker dialogue; raw askme event logs were not addressed and kan read failures collapsed to empty.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidgup6wlaiathlyyqk3fghklczgclz3xrfonqfbpr45s674uz34b4",
  "sig": "4702d408d1d2d840f699edc2e55139216b6801ab9803e3a5d2516d0793fd22f5069f53ed2671be721a7dcc530398c2c6974d0609852df890e4be97446efbbe8e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreihzy65oajegvv5gd6tfrwgxh4kpyaifstqp66m4jqr3z62a7bx2hy"
  ],
  "rev": "223mtcoaxhbgq",
  "seq": 85,
  "of": 125,
  "text_len": 130,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg+ce64CSGrXph+mWNjXPxT8AQWU4P95nEwjvPtA+G+j5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0NTdmZTQyZTcwYmE0Yzg4MzIwZDYxMzQ5NjQ5YTM1ODVmMzlkOGVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUUN1p0h"
}
---

Reconstruction grading accepted arbitrary token-containing text instead of parsed bulk-kan claims and typed fresh-wakeup evidence.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidwgwa4p5mowalho5bsvpr4lviv5cezudrppsrsj5oc2vskijwecq",
  "sig": "51ea284cc66f0e3a3f884a3f700806cee8112d654aef7ac43d59cbe57dc76aee6c78cefb13f0eee7be31e7919768b65d781e8060f729b0bc9429ae4368067712",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreihzy65oajegvv5gd6tfrwgxh4kpyaifstqp66m4jqr3z62a7bx2hy"
  ],
  "rev": "223mtcoay77nb",
  "seq": 86,
  "of": 125,
  "text_len": 129,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg+ce64CSGrXph+mWNjXPxT8AQWU4P95nEwjvPtA+G+j5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0NTdmZTQyZTcwYmE0Yzg4MzIwZDYxMzQ5NjQ5YTM1ODVmMzlkOGVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUUN4pXy"
}
---

Transcript rejection admitted named-speaker dialogue such as Alice:/Bob: because it recognized only a fixed role-label allowlist.
***8<***
---
{
  "v": 3,
  "cid": "bafyreighk3sr6f5rdaoz3qye4xalal6ppzo45y6ogqeeyxskbuw752z42u",
  "sig": "85afeef8ebd665897b3d22a3da00a526e1535260ec68185fa73f6d382871ce0d407d1ddcab301bb732da65e064f4492c842468a11ac19ae0442ee682f98cefec",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreihzy65oajegvv5gd6tfrwgxh4kpyaifstqp66m4jqr3z62a7bx2hy"
  ],
  "rev": "223mtcoaywxce",
  "seq": 87,
  "of": 125,
  "text_len": 114,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg+ce64CSGrXph+mWNjXPxT8AQWU4P95nEwjvPtA+G+j5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0NTdmZTQyZTcwYmE0Yzg4MzIwZDYxMzQ5NjQ5YTM1ODVmMzlkOGVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUUN7nST"
}
---

The askme evidence manifest omitted raw Codex JSONL digests and normalized failed kan reads to an empty-log shape.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic4iayl3y3frmpgdooqvyhtejjf2gvbifvlfgot3iwlt7vhkphrxi",
  "sig": "61a495ec1e07b321e4260cd0f68bbc702c2da124b0ad9e66b467a63ab4d0c31767d3ce2464bc8e513338373de3303ab57feb56f32c0fed04b69543f88668f979",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mtcob6pwxl",
  "seq": 88,
  "of": 125,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4GhZkNvbW1pdHgoNDU3ZmU0MmU3MGJhNGM4ODMyMGQ2MTM0OTY0OWEzNTg1ZjM5ZDhlY2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABllFDkrzPg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibng5aytmhla33einxjowlv2yi4klvakhvkmfhzjjdkqdkpwh6nke",
  "sig": "9ce95413b1494a92a4c096ef6ea412e71f69a3b5fd308e92866d56142f0c2b1252e04ba79eff9bad4461d1b05d38ca7de2065ab38ba9f6adf5936b3b501943c1",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreifpbz6x3kaozus5ayuftdwrewrkfcdh4x6j7ecsfvu55w665o7ykm"
  ],
  "rev": "223mtcqobrtd7",
  "seq": 89,
  "of": 125,
  "text_len": 297,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrw59fagOzSXQYoWY7RJaKiiGfl/J+QUi1p3tve67+FNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkZWNhMjA1ZDMwMDYxOWNmNmNiMjU5NjY0ODJiZDQ5MGViODNjZDhmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUWoe+S/"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Cold review round 2: reconstruction accepted self-authored typed evidence, reported provenance admitted named-speaker transcripts, and askme raw logs were digest-consistent without typed Codex-event derivation; return to generative build.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihcvgx2pbl5teo776de3dnn4k2e2u5bstkrvb7fple72df4ir6664",
  "sig": "eb63383302d51917a9c5de8ca0621500fbbefcc81d51ded0bffcaa71a27e59ce18f76c3fafceee776041ac30a1a6150ca475583b4e79461a25abc729f3b358b0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreib3t4pfqgs7x6sfsbb7fjzgykr7ilk75t2idl7dqdbmqn2aa7agie"
  ],
  "rev": "223mtcqozfa4b",
  "seq": 90,
  "of": 125,
  "text_len": 200,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgO58eWBpfv6RZBD8qcmwqP0LV/s9IGv44DCyDdAB8BkFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0ZWY1YzIzMWRiOWZkNGU2MWVmZDlhOTUyYmNjNWZmNDY2OWExZTE1aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUWp9ZfX"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 18095:bba9a0542160b81e]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihzwhywvptruf24ymig22jihbmrv3lak3gm2niwjdmrsgle2m4lvq",
  "sig": "f3f24c1b11928f0014603dc3a1b3ce5a4bbd0474225ac7d46d94ca24778828b725fe8cba59ec35ab8c70f92ca2808f51f336ba02acaa4dd99aa9b1258c9588b1",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreihcvgx2pbl5teo776de3dnn4k2e2u5bstkrvb7fple72df4ir6664",
    "bafyreigwitghalchjvnytvfkyj5cglvcpoo6cjivt2vnaeopwd6cw4klay",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreifapex5udoki4mzdd2q37qjaudjljj7cw6smnjj2qum6cbrojll5q"
  ],
  "rev": "223mtcqp24ypl",
  "seq": 91,
  "of": 125,
  "text_len": 856,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4TYKlglAAFxEiDiqa+nhX2ZHf/4ZNja3itE1ToZTVGoflesn9DLxEfe99gqWCUAAXESINZEzHAsR01bidSqwnojLqJ7neElFZ6q0BHPsPwrcUsG2CpYJQABcRIgSLW7VoFxdyRc26vflJ9ARpzM2XdTZ10LSPqvPunGG6DYKlglAAFxEiCgeS/aDcpHGZGPUN/gkFBpWlPxW9JjUp1CjPCDFyVr7GZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDRlZjVjMjMxZGI5ZmQ0ZTYxZWZkOWE5NTJiY2M1ZmY0NjY5YTFlMTVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRaoBej8="
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143, #152, #193, #195, and #204. It makes interrupted work cheaper to recover and human direction more precise to record without turning day into a tracker or a transcript. The release distinguishes post-compaction startup, retains the newest bounded practice, gives handoff measurements immutable executable coordinates, inventories visible handoff streams from one honest bulk read, ships general `/askme`, and records acquired input and interventions only at an explicit write boundary. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)] [normative citations: bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua,bafyreifapex5udoki4mzdd2q37qjaudjljj7cw6smnjj2qum6cbrojll5q]
***8<***
---
{
  "v": 3,
  "cid": "bafyreib4rxmjcm3phgfqo7jdv3dkz7tgcnxfa3v6a3jcbq5smwyt24atiy",
  "sig": "5a8b897ffc4769183de660fe887cb1d75c14561c4fe1550ac43efb3c640a7e1e13b186d0447e0e07a4248eb376fb9a82c51a6faf84128a2d7f918f5ae03c9517",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtcqp2lhaj",
  "seq": 92,
  "of": 125,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2UmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljcywgY29ycmVjdGlvbiByb3VuZCA1bHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDRlZjVjMjMxZGI5ZmQ0ZTYxZWZkOWE5NTJiY2M1ZmY0NjY5YTFlMTVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRaoItGo="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiafzx65l5bkxzadmqtdmxsfkwdcg6b75k5s4u76flemyy7h3mvoxq",
  "sig": "e04e22043ef6776d684a28785f3bf2227c8aba43076635b967a23277a75e67051e9abe9a37bfe7d0da9bbde67d1674b8e272471f685898998b374237a5cf755c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcqpoonog",
  "seq": 93,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0ZWY1YzIzMWRiOWZkNGU2MWVmZDlhOTUyYmNjNWZmNDY2OWExZTE1aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUWrSk4a"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaegqit5tmgxrcf5q2272svdjbsvevmpzh6e6x2k5fpnhgjxz3czm",
  "sig": "4b26cd5bec8cecb37f3f156a480345229b852b5124ad01ad853dcb071f81178d279d7d8852e7afbc192916ba07d3024bca59de23e468ac6e4d00e86aa924b3e3",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreihzwhywvptruf24ymig22jihbmrv3lak3gm2niwjdmrsgle2m4lvq",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreifapex5udoki4mzdd2q37qjaudjljj7cw6smnjj2qum6cbrojll5q"
  ],
  "rev": "223mtcqqmk5qv",
  "seq": 94,
  "of": 125,
  "text_len": 18095,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiD5sfFqvnGhdcwxBtaSg4WRrtYFbMzTUWSNkZGWTTOLrNgqWCUAAXESIEi1u1aBcXckXNur35SfQEaczNl3U2ddC0j6rz7pxhug2CpYJQABcRIgoHkv2g3KRxmRj1Df4JBQaVpT8VvSY1KdQozwgxcla+xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCg0ZWY1YzIzMWRiOWZkNGU2MWVmZDlhOTUyYmNjNWZmNDY2OWExZTE1oWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDRlZjVjMjMxZGI5ZmQ0ZTYxZWZkOWE5NTJiY2M1ZmY0NjY5YTFlMTVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRa0oDl4="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, harness version, and file digests. The repository grader recomputes
  each outcome from addressed evidence; changing a transcript while updating
  its digest must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. The workflow itself launches the fresh Codex session, derives
  its typed raw-event and kan-read bundle, and the grader binds the signed
  claims and suite/census/CI coordinates to one candidate before deriving every
  removal and wrong-candidate control.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under explicitly named signing principals, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, or transcript content in durable claims.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate. All eleven registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreib67e5um4pjx6g4h4a7psmkdiemzg6swuexwlooir2iuciryoltda",
  "sig": "8e464255f0d698d8013ba3e495fa280dce1db58775804cf71bd2b53917e6706d20303207c1f74c6cd984f642afb34fbf5b8fc678f6079829f24fe120f532c62a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcqqwls77",
  "seq": 95,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0ZWY1YzIzMWRiOWZkNGU2MWVmZDlhOTUyYmNjNWZmNDY2OWExZTE1aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUWtyOAv"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiczp4u7hmaf7ao5s7vtawfwoyioihn6ysrh5lby5zjep6tgvgsomq",
  "sig": "caf600a67aa7f8268e4e84e1668e1beea8fe80561e2ec540f82f95b0190f6e437cb1e6882b057964ae716e991df989fb711fdc7d324f08c0c8270a4e26e90dd0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaegqit5tmgxrcf5q2272svdjbsvevmpzh6e6x2k5fpnhgjxz3czm"
  ],
  "rev": "223mtcs3kp5hd",
  "seq": 96,
  "of": 125,
  "text_len": 324,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBDQRPs2GvERew1r+pVGkMqkqx+T+J6+ldK9pzJvnYstmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4YzZlMTUzMjJmZmZlNDlmOTg2ZGY0MTY4YWQ1YmI2OGE1NDVkOTgzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUYDCo00"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Cold review round 3: positional work subjects bypassed the transcript invariant; askme structural fixtures were not workflow-origin bound; reconstruction admitted self-selected signing principals without signed adjudication or raw rechecks of suite, census, and CI.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaioys2c6w5nuo4m7cep2lhe3bpxorbcl4naohqabdygzfwhqn43u",
  "sig": "a2485c37fac45a3d1f374bfbfef0d6d47e4cd4fb3c50e53389a6ba513925601b082e364c012603b2ab80be00d55fe9a40e91bdb8c866105cea32fe9ec4d92e19",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiaegqit5tmgxrcf5q2272svdjbsvevmpzh6e6x2k5fpnhgjxz3czm",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreifapex5udoki4mzdd2q37qjaudjljj7cw6smnjj2qum6cbrojll5q"
  ],
  "rev": "223mtcs4kg2zl",
  "seq": 97,
  "of": 125,
  "text_len": 18303,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiAENBE+zYa8RF7DWv6lUaQyqSrH5P4nr6V0r2nMm+diy9gqWCUAAXESIEi1u1aBcXckXNur35SfQEaczNl3U2ddC0j6rz7pxhug2CpYJQABcRIgoHkv2g3KRxmRj1Df4JBQaVpT8VvSY1KdQozwgxcla+xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eChhYzRkZjNiOWJlZDM4YTg0YjA4ZjMxOTBiZjNiZmY4ZmM3NjZjM2EwoWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KGFjNGRmM2I5YmVkMzhhODRiMDhmMzE5MGJmM2JmZjhmYzc2NmMzYTBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRgUGA20="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. The public grader
  is authoritative only inside that exact candidate workflow and recomputes
  each outcome from addressed typed Codex events; changing a transcript while
  updating its digest must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session, derives its typed raw-event and kan-read
  bundle, and requires successful command events that recheck suite, census,
  and CI coordinates before deriving every removal and wrong-candidate control.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, or transcript content in durable claims.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate. All eleven registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreiasjmhnzw6aqkffwv4fd7ocqs65k362pmfpybsa2v3al7yqilisyu",
  "sig": "86889ce329e2911a14a40e4f5b234b873c08913766acaf13515acdffd9b9e58956e7d1914ed7f5b4991e1f543c9efad14cbe9910025d6da2c06ca1c4485487fd",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcs4y5gff",
  "seq": 98,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChhYzRkZjNiOWJlZDM4YTg0YjA4ZjMxOTBiZjNiZmY4ZmM3NjZjM2EwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUYF4bD1"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibgkfdwqtzfesf3x4cxdwx3u4ksu5zso2otyszazyf7kzmovorj24",
  "sig": "03b698d3b5fdc73eb711bd28b98cc7a6c8d2cdbb39227f74e5eb8e2493bb4e0e565861b276f2b0168ed3721e009727008e768fee9d4cdfe53695f5cf04483e0e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "observation",
  "cites": [
    "bafyreihcvgx2pbl5teo776de3dnn4k2e2u5bstkrvb7fple72df4ir6664"
  ],
  "rev": "223mtcthxwsz4",
  "seq": 99,
  "of": 125,
  "text_len": 200,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg4qmvp4V9mR3/+GTY2t4rRNU6GU1RqH5XrJ/Qy8RH3vdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUZb3l+s"
}
---

design doc .design/v0.13-workflow-ergonomics.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 19295:a8e37ce8f5d18772]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigpapcymvvp7flljpoepwtmbamfet2pfp742267cmycvwa6yk4dvi",
  "sig": "16190d4bace0b60d994649780968854b83199a3099851b7f25447786ab8f1ccf5a0d34e5251d31825eca69c5752e7ed8c5aec9b77c3c806b387bc9774f336880",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreibgkfdwqtzfesf3x4cxdwx3u4ksu5zso2otyszazyf7kzmovorj24",
    "bafyreihzwhywvptruf24ymig22jihbmrv3lak3gm2niwjdmrsgle2m4lvq",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua"
  ],
  "rev": "223mtcthyxohr",
  "seq": 100,
  "of": 125,
  "text_len": 796,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiAmUUdoTyUki7vwVx2vunFSp3MnadPEsgzgv1ZY6rop19gqWCUAAXESIPmx8Wq+caF1zDEG1pKDhZGu1gVszNNRZI2RkZZNM4us2CpYJQABcRIgSLW7VoFxdyRc26vflJ9ARpzM2XdTZ10LSPqvPunGG6BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUZb7tEj"
}
---

v0.13-workflow-ergonomics design (.design/v0.13-workflow-ergonomics.md): `v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143, #152, #193, #195, and #204. It makes interrupted work cheaper to recover and human direction more precise to record without turning day into a tracker or a transcript. The release distinguishes post-compaction startup, retains the newest bounded practice, gives handoff measurements immutable executable coordinates, inventories visible handoff streams from one honest bulk read, ships general `/askme`, and records acquired input and interventions only at an explicit write boundary. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)] [normative citations: bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidk6emxtgqb3g4rpr7glyd77snxfplkb74ewyyz7sk4m2gcc4xhr4",
  "sig": "d90365ead3dcbeabb86e2d922d39990797fcca8ee94f85698995b018c42409bd6a3b92996066a4e7c9f138943711b480d5c11a3caab3f548ac1c75163f3d8db7",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtcthzkwlf",
  "seq": 101,
  "of": 125,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2UmVsZWFzZTogdjAuMTMgd29ya2Zsb3cgZXJnb25vbWljcywgY29ycmVjdGlvbiByb3VuZCA1bHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHgZdjAuMTMtd29ya2Zsb3ctZXJnb25vbWljc2lhcnRpZmFjdHOBoWZDb21taXR4KDc1OGQzNjc5ZDlmMTU1ODljZTFlYjJkMTk0MGI5NjAwMDE4MTg1MGZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRlv4caU="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreieftq5dydoc4en6revtopqdwrlchac7og4byrskt2ewgzigo5p6ba",
  "sig": "88b5078e9a516a2dff098cde8d952b2466588b863e86523663337dff087196db1ea85de389385db5588f4c193d6f4d584e41b9b75637558fbecf22a5332cee5d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcti7za65",
  "seq": 102,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUZcX5eM"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigzb6mbjloskfuti5vngfvjluvn3f7wk7aytculn4t6vrpveufbg4",
  "sig": "e96c9c047007ada512cd1d15bce95c37cf4c4cbc702569c3dd5c27cceb41c14a226ea2ef3fc29924917a6bb1582f5ce7ac1d68cea34becfd5518e19a5c1f60a2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaioys2c6w5nuo4m7cep2lhe3bpxorbcl4naohqabdygzfwhqn43u"
  ],
  "rev": "223mtctjypajt",
  "seq": 103,
  "of": 125,
  "text_len": 436,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCHYloXrdbR3GfER+lnJsL7uiES+NA48ABHg2S2PBvN1mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUZf6pmD"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Round 4 found punctuation-form multi-speaker transcripts remained appendable, the real Codex workflows authenticated outside the isolated execution home, reconstruction accepted comment-disguised verification commands and incomplete event ordering, and review Decisions could shadow the telos declaration. Fix commit 758d367 closes these defects; a new cold review is required.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid6po2te3qbppsrzfwlxv4jtqxaumqgtt6nslr7f5kw6pdpkyss6a",
  "sig": "3ea8e2e289cfd0c3a5df6b24e3bdbd20347142efd9dbc844b603a1e17266d81b0a8b2c1e5721ec78e4e5c49de320f525e961cadcd56fd829205e0f063e78063a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreibgkfdwqtzfesf3x4cxdwx3u4ksu5zso2otyszazyf7kzmovorj24",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreigzb6mbjloskfuti5vngfvjluvn3f7wk7aytculn4t6vrpveufbg4"
  ],
  "rev": "223mtctk7d5il",
  "seq": 104,
  "of": 125,
  "text_len": 19294,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiAmUUdoTyUki7vwVx2vunFSp3MnadPEsgzgv1ZY6rop19gqWCUAAXESIEi1u1aBcXckXNur35SfQEaczNl3U2ddC0j6rz7pxhug2CpYJQABcRIg2Q+YFK3SUWk0dq0xapXSrdl/ZXwYmKi28n6sX1JQoTdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmoWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDc1OGQzNjc5ZDlmMTU1ODljZTFlYjJkMTk0MGI5NjAwMDE4MTg1MGZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRmBUjU4="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, and dash-dialogue forms;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside that exact candidate workflow, requires
  closed thread/turn/item lifecycle ordering, and recomputes each outcome from
  addressed typed Codex events; changing a transcript while updating its digest
  must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session, derives its typed raw-event and kan-read
  bundle, and requires successful command events whose command strings exactly
  equal the recovered suite, census, and CI invocations. Comments, shell
  prefixes, and output-only lookalikes do not certify a recheck. Every removal
  and wrong-candidate control is then derived from that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, and dash-dialogue
      variants; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned closed lifecycle. The authenticated
      Codex home is the same isolated home used to execute every scenario.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully. All twelve registered
      removal/wrong-candidate controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig3gpf2aukt26v2fig426aief2pyffngavyxmg33h6d5gbhhnh7ee",
  "sig": "88070f19c89b08e67604d48801a217c8055d3de5be280497d6857f7482da0e346826a1dea61cb1ced038576dba99ebbb3937c028ac90c142437f413e43954005",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtctkdegel",
  "seq": 105,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUZglTDZ"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigtlu7ehhggfwia2mjvxkcore5a26vkxnwibjs5dmlntgmqhuthga",
  "sig": "b1e825ec3021a25fa25913322804241c1a1339b465acd0b84f55d0de9f3cb1331bdd29616cdeed4ea42e65bbd3c9736eca6d7fc93e9fc9d025015d4dcde955c9",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreibgkfdwqtzfesf3x4cxdwx3u4ksu5zso2otyszazyf7kzmovorj24",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreigzb6mbjloskfuti5vngfvjluvn3f7wk7aytculn4t6vrpveufbg4"
  ],
  "rev": "223mtctky7sfn",
  "seq": 106,
  "of": 125,
  "text_len": 19295,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiAmUUdoTyUki7vwVx2vunFSp3MnadPEsgzgv1ZY6rop19gqWCUAAXESIEi1u1aBcXckXNur35SfQEaczNl3U2ddC0j6rz7pxhug2CpYJQABcRIg2Q+YFK3SUWk0dq0xapXSrdl/ZXwYmKi28n6sX1JQoTdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmoWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDc1OGQzNjc5ZDlmMTU1ODljZTFlYjJkMTk0MGI5NjAwMDE4MTg1MGZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRmHi4O8="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, and dash-dialogue forms;
  malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside that exact candidate workflow, requires
  closed thread/turn/item lifecycle ordering, and recomputes each outcome from
  addressed typed Codex events; changing a transcript while updating its digest
  must still fail the rubric when behavior changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session, derives its typed raw-event and kan-read
  bundle, and requires successful command events whose command strings exactly
  equal the recovered suite, census, and CI invocations. Comments, shell
  prefixes, and output-only lookalikes do not certify a recheck. Every removal
  and wrong-candidate control is then derived from that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, and dash-dialogue
      variants; malformed and ambiguous source requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned closed lifecycle. The authenticated
      Codex home is the same isolated home used to execute every scenario.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully. All twelve registered
      removal/wrong-candidate controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreihvyabpmfa2c42nbrjvi2mrquiajajhf5fcpva6jsenwkiupalif4",
  "sig": "1cd27e5cebe3c4c22154b14c4e41482f4a8b708db4e6bb201e52a8a13962cc083399790efd517fa111b800899a185906aa9f8a3a1c05d1894f40b9dcad1abda6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtctl4f35m",
  "seq": 107,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NThkMzY3OWQ5ZjE1NTg5Y2UxZWIyZDE5NDBiOTYwMDAxODE4NTBmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUZiJYP8"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiem2o2pq5ojc5kmnd7jf2hyntf6bsdmrndw3w5ksyqptsuez7sdvq",
  "sig": "fa88c6847042b63afe0b8657afcb3ac59f7c07e5018a6081a4245497d179022e3a10ca7f23b4b428ce760823963629820a1bf1dae6076eb8d6b12d70c1dd42e2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreigtlu7ehhggfwia2mjvxkcore5a26vkxnwibjs5dmlntgmqhuthga"
  ],
  "rev": "223mtcvfxwymh",
  "seq": 108,
  "of": 125,
  "text_len": 477,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg010+Q5zGLZANMTW6hOiToNeqq7bICmXRsW2ZmQPSZzBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ODE3MmNlZjMwYzJmNzllYzVlMGYzNmE4ZDcyZjA1MjFmNTM1NmEwaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUbX3nnY"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Round 5 found compact Unicode-dash dialogue still appendable while ordinary labeled summaries were falsely rejected; grader origin checks did not bind the kan-tools/day repository and exact workflow file; Codex item IDs could change type across one lifecycle; and reconstruction ran recovered suite commands from the evidence checkout rather than proving the candidate checkout. A new fix and cold review are required.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihvbdeikkphy62x2msms6y3yfe34e5clrqn3ef7txuh7c2eea5bxi",
  "sig": "5efcc3b7ef4ea6c70a8bb184770f9899d4253c4cfb0eac178d29f76a419a2cf05d8aec62ccda59cd5c6690fe4cf2a0d34cdcfd0add3e0c4ff889cdde102aaa96",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreigtlu7ehhggfwia2mjvxkcore5a26vkxnwibjs5dmlntgmqhuthga",
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreiem2o2pq5ojc5kmnd7jf2hyntf6bsdmrndw3w5ksyqptsuez7sdvq"
  ],
  "rev": "223mtcvrkvdmr",
  "seq": 109,
  "of": 125,
  "text_len": 19988,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiDTXT5DnMYtkA0xNbqE6JOg16qrtsgKZdGxbZmZA9JnMNgqWCUAAXESIEi1u1aBcXckXNur35SfQEaczNl3U2ddC0j6rz7pxhug2CpYJQABcRIgjNO0+HXJF1TGj+kuj4bMvgyGyLR23bqpYg+cqEz+Q6xmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCgzN2E1OWYyMjNmMTNjZjdiYTYwODk3MTU5NjE5ODMwNmI5OGY2OTk0oWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDM3YTU5ZjIyM2YxM2NmN2JhNjA4OTcxNTk2MTk4MzA2Yjk4ZjY5OTRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZRu8NpdE="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, and spaced or compact Unicode
  dash-dialogue forms. The approximation distinguishes speaker roles/proper
  names from ordinary structured labels such as Decision/Effect and
  Risk/Mitigation; malformed or ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside `kan-tools/day` on `github.com`, from the
  exact registered workflow file and `workflow_dispatch` event at that candidate
  and run. It requires closed, type-stable thread/turn/item lifecycle ordering
  and recomputes each outcome from addressed typed Codex events; changing a
  transcript while updating its digest must still fail the rubric when behavior
  changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session from the clean candidate checkout while a
  narrow wrapper routes kan reads to the evidence checkout. Its typed raw-event
  bundle must independently show exact successful `git rev-parse HEAD` and
  empty `git status --porcelain` commands before exact recovered suite, census,
  and CI invocations. Comments, shell prefixes, wrong item types, and output-only
  lookalikes do not certify a recheck. Every removal and wrong-candidate control
  is then derived from that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, and spaced/compact
      Unicode-dash variants. Ordinary Decision/Effect, Risk/Mitigation, and RFC
      source/scope summaries remain accepted. Malformed and ambiguous source
      requests likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned type-stable lifecycle, or a grader
      origin outside the exact repository/workflow-file/event tuple. The
      authenticated Codex home is the same isolated home used for every scenario.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully from an independently
      rechecked clean checkout at the candidate SHA. All twelve registered
      removal/wrong-candidate controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreigdpygmypwjdkjmiz2l3rdutjk3rdndnrhl7vina3xwhgafioqumy",
  "sig": "2a8931242accb8c7ee87824b7d53a4f94e6830e14bd2c269cdef7dd522fc0cf415ba58b4bf025b812c1c7779b48bf63217f3b005e36f9fca767a9f32c99b3eee",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcvrp7agj",
  "seq": 110,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgzN2E1OWYyMjNmMTNjZjdiYTYwODk3MTU5NjE5ODMwNmI5OGY2OTk0aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUbvUpkP"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibjxa2advt5rt2iswr4xqtzrwjpnajtikhgknkovi3ewoeshy3sxe",
  "sig": "7c8c391f27920675c11403e7d3957e89daf13d465dc3d2039208a15f611ff3bc2427fdde93f742175f636f50428a6e39a1842f635aec8dc55ac8186b62e8c094",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreihvbdeikkphy62x2msms6y3yfe34e5clrqn3ef7txuh7c2eea5bxi"
  ],
  "rev": "223mtcwn6q2cp",
  "seq": 111,
  "of": 125,
  "text_len": 419,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9QjIhSnnx7V9MkyXsbwUm+E6JcYN2Qv53of4tEIDobpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgyNmIxOTIxOWVkYWI0M2JiOWNmYmNiZDcwZGJkOThiZjI2YWZjMjc1aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUcmSwCc"
}
---

adversarial review of v0.13-workflow-ergonomics: BLOCK — Round 6 found standalone Markdown speaker headings appendable and Q1/AC1 structured labels falsely rejected; reconstruction command evidence was unordered, cwd-unbound, PATH-redirectable from gitignored target/debug, and census/CI outputs were substring-checked. Workflow ref/run origin binding also remained incomplete. A new fix and cold review are required.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidbqk3gxvwlz3xry5ohuyrewyptripugx2hmnv6tnwpv6fvnvquhq",
  "sig": "a1fc783b15a2a71ff4eb4eab4834f033d5b8ec6bf1dee79d9e59b5ead97102ea3689d958080746dc9cab51a472928e585ce1ab304eeaebcc8ef7a657c7cb19c3",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreihvbdeikkphy62x2msms6y3yfe34e5clrqn3ef7txuh7c2eea5bxi",
    "bafyreibjxa2advt5rt2iswr4xqtzrwjpnajtikhgknkovi3ewoeshy3sxe"
  ],
  "rev": "223mtcx2yh4ne",
  "seq": 112,
  "of": 125,
  "text_len": 20605,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiD1CMiFKefHtX0yTJexvBSb4Tolxg3ZC/neh/i0QgOhutgqWCUAAXESICm4NAHWfYz0iVo8vCeY2S9oEzQo5lNU6qNks4kj43K5ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseBl2MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzaWFydGlmYWN0c4KhZkNvbW1pdHgoNDU0ZDIxNzgyYjFhYzNlYjg2ODM5OGU1MTg2NTBlYzJhZTQ1OTc4ZqFmRmlsZUF0gngkLmRlc2lnbi92MC4xMy13b3JrZmxvdy1lcmdvbm9taWNzLm1keCg0NTRkMjE3ODJiMWFjM2ViODY4Mzk4ZTUxODY1MGVjMmFlNDU5NzhmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUdB5oXg"
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, Markdown-only speaker headings,
  and spaced or compact Unicode dash-dialogue forms. The approximation
  distinguishes speaker roles/proper names from ordinary structured labels
  such as Decision/Effect, Risk/Mitigation, Q1/Q2, and AC1/AC2; malformed or
  ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside `kan-tools/day` on `github.com`, from the
  exact registered workflow file/ref, workflow SHA, and `workflow_dispatch`
  event at that candidate and run. It requires closed, type-stable
  thread/turn/item lifecycle ordering
  and recomputes each outcome from addressed typed Codex events; changing a
  transcript while updating its digest must still fail the rubric when behavior
  changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session from the clean candidate checkout. Root-owned
  wrappers outside the model-writable checkout pin the real executables, reject
  the wrong working directory, HEAD, or tree state immediately before each
  command, and route kan reads to the evidence checkout. Its typed raw-event
  bundle must independently show exact successful `git rev-parse HEAD`, empty
  `git status --porcelain`, bulk kan, recovered suite, census, and CI invocations
  in that order. Census rows and CI JSON are parsed structurally. Comments,
  shell prefixes, wrong item types, malformed output, and output-only lookalikes
  do not certify a recheck. The generated bundle also binds the executing
  GitHub run. Every removal and wrong-candidate control is then derived from
  that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, Markdown speaker
      headings, and spaced/compact Unicode-dash variants. Ordinary
      Decision/Effect, Risk/Mitigation, Q1/Q2, AC1/AC2, and RFC source/scope
      summaries remain accepted. Malformed and ambiguous source requests
      likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned type-stable lifecycle, or a grader
      origin outside the exact repository/workflow-file/event tuple. The
      authenticated Codex home is the same isolated home used for every scenario.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully in order through
      root-owned executable-pinning wrappers from an independently rechecked
      clean checkout at the candidate SHA. Census and CI output is structurally
      exact, and the generated bundle run ID plus workflow file/ref/SHA match
      the executing upstream run. All twelve registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreicc34tcysqtapj6xa65cfqirvy5sb3tuuavqud27t5ccwc4bax73a",
  "sig": "8a595e2fb0010900fbdae27d1995fd7650f145b0a342a6cfd1ec7eedce1a5f1444dac2a2421f54cd2474c54d92d7cad8247e314b530a7f63f94c2cdfa3c5811c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcx3buwbu",
  "seq": 113,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0NTRkMjE3ODJiMWFjM2ViODY4Mzk4ZTUxODY1MGVjMmFlNDU5NzhmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUdCfXB8"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidcjsugbortqb7kket36tkfvrkbjo5da462fl67nyr25f44auze2y",
  "sig": "5ff1d1d9349a9b6fd3988dc1fd460ee2e1246fb717323befd32f4dd580c7ea1f6fc3671c6046436a7e29d430ed9262db5e25455d36fd132c0b1302fed6195f1b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreidbqk3gxvwlz3xry5ohuyrewyptripugx2hmnv6tnwpv6fvnvquhq",
    "bafyreibjxa2advt5rt2iswr4xqtzrwjpnajtikhgknkovi3ewoeshy3sxe"
  ],
  "rev": "223mtcx4n36ah",
  "seq": 114,
  "of": 125,
  "text_len": 20605,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboNgqWCUAAXESIGGCtmvWy87vHHXHpiJLYfOKH0NfR2Nr6bbPr4tW1hQ82CpYJQABcRIgKbg0AdZ9jPSJWjy8J5jZL2gTNCjmU1Tqo2SziSPjcrlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCg0NTRkMjE3ODJiMWFjM2ViODY4Mzk4ZTUxODY1MGVjMmFlNDU5NzhmoWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDQ1NGQyMTc4MmIxYWMzZWI4NjgzOThlNTE4NjUwZWMyYWU0NTk3OGZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZR0UwkEI="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, Markdown-only speaker headings,
  and spaced or compact Unicode dash-dialogue forms. The approximation
  distinguishes speaker roles/proper names from ordinary structured labels
  such as Decision/Effect, Risk/Mitigation, Q1/Q2, and AC1/AC2; malformed or
  ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside `kan-tools/day` on `github.com`, from the
  exact registered workflow file/ref, workflow SHA, and `workflow_dispatch`
  event at that candidate and run. It requires closed, type-stable
  thread/turn/item lifecycle ordering
  and recomputes each outcome from addressed typed Codex events; changing a
  transcript while updating its digest must still fail the rubric when behavior
  changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session from the clean candidate checkout. Root-owned
  wrappers outside the model-writable checkout pin the real executables, reject
  the wrong working directory, HEAD, or tree state immediately before each
  command, and route kan reads to the evidence checkout. Its typed raw-event
  bundle must independently show exact successful `git rev-parse HEAD`, empty
  `git status --porcelain`, bulk kan, recovered suite, census, and CI invocations
  in that order. Census rows and CI JSON are parsed structurally. Comments,
  shell prefixes, wrong item types, malformed output, and output-only lookalikes
  do not certify a recheck. The generated bundle also binds the executing
  GitHub run. Every removal and wrong-candidate control is then derived from
  that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, Markdown speaker
      headings, and spaced/compact Unicode-dash variants. Ordinary
      Decision/Effect, Risk/Mitigation, Q1/Q2, AC1/AC2, and RFC source/scope
      summaries remain accepted. Malformed and ambiguous source requests
      likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned type-stable lifecycle, or a grader
      origin outside the exact repository/workflow-file/event tuple. The
      authenticated Codex home is the same isolated home used for every scenario.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully in order through
      root-owned executable-pinning wrappers from an independently rechecked
      clean checkout at the candidate SHA. Census and CI output is structurally
      exact, and the generated bundle run ID plus workflow file/ref/SHA match
      the executing upstream run. All twelve registered removal/wrong-candidate
      controls are derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreigx6x2lswb25li2ktahubx3z5kyc5tqri2mkuc2ywpdtmec2cie6y",
  "sig": "c099a009102eef89104f20997d01bee952712470fbb0935b110900eab8482adc7d79a24926b0422a30a4b7c65e547f0167ecb682c8a7c033b7b1d7a673834527",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcx4nxb35",
  "seq": 115,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0NTRkMjE3ODJiMWFjM2ViODY4Mzk4ZTUxODY1MGVjMmFlNDU5NzhmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUdFPpuq"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiclijcs637lgp6n4cwfnryumpxeukl5scp3k4umdiyrmobo7u3n5i",
  "sig": "78a6de1d4ae1f065fd1ae13a8248ee158f850f503110b27052a1dfa60da06f4c4cc5ec74c14918b7c672a4c2b6ad7c1e89da07bc6ef6cd1ea80d3d8b783b988a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreidcjsugbortqb7kket36tkfvrkbjo5da462fl67nyr25f44auze2y"
  ],
  "rev": "223mtcykrnz4i",
  "seq": 116,
  "of": 125,
  "text_len": 456,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgYkyoYLozgH6lEnv01FrFQUu6MHPaKv324jrpecBTJNZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkMmIwNGU2NTk4ZjQ4ZGNmYmNmMjBjMzFhMmNlOTY2ZDA5YThhZmE3aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUehefvY"
}
---

adversarial review of v0.13-workflow-ergonomics at d2b04e6598f48dcfbcf20c31a2ce966d09a8afa7: BLOCK — multiword Markdown proper-name speaker headings cross the durable transcript boundary, and model-writable gitignored Cargo artifacts can replace cached test executables while exact trusted-wrapper commands still certify success. REQ-7/AC-8 and REQ-11/AC-12 remain unmet; return to generative-build, add hostile end-to-end rejection tests, and re-review.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic3ttqzqsn4omku54cwu6vnqqqwlndkqotubog72yunjorjqfbkru",
  "sig": "f30a9a8389b21bca45ef28eca9eb926ac8166444008d297d41212be3c0d7addc108264831ea77fab38dec05556318170b5ee9f22f8c334e66a6bdee2e75056bd",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreidcjsugbortqb7kket36tkfvrkbjo5da462fl67nyr25f44auze2y",
    "bafyreiclijcs637lgp6n4cwfnryumpxeukl5scp3k4umdiyrmobo7u3n5i"
  ],
  "rev": "223mtcyu3bhpx",
  "seq": 117,
  "of": 125,
  "text_len": 21091,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboNgqWCUAAXESIGJMqGC6M4B+pRJ79NRaxUFLujBz2ir99uI66XnAUyTW2CpYJQABcRIgS0JFL2/rM/zeCsVscUY+5KKX2Qn7VyjBoxFjgu/TbepmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCg1Mzk4MWY4MDEzM2JhYmZlYzUxMmJjMWFkYTE2YzQxYjY4ODBhN2JkoWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDUzOTgxZjgwMTMzYmFiZmVjNTEyYmMxYWRhMTZjNDFiNjg4MGE3YmRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZR7QTtkA="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, single- or multiword Markdown
  proper-name speaker headings, and spaced or compact Unicode dash-dialogue
  forms. The approximation
  distinguishes speaker roles/proper names from ordinary structured labels
  such as Decision/Effect, Risk/Mitigation, Q1/Q2, and AC1/AC2; malformed or
  ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside `kan-tools/day` on `github.com`, from the
  exact registered workflow file/ref, workflow SHA, and `workflow_dispatch`
  event at that candidate and run. It requires closed, type-stable
  thread/turn/item lifecycle ordering
  and recomputes each outcome from addressed typed Codex events; changing a
  transcript while updating its digest must still fail the rubric when behavior
  changes.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session from the clean candidate checkout. Root-owned
  wrappers outside the model-writable checkout pin the real executables, reject
  the wrong working directory, HEAD, or tree state immediately before each
  command, and route kan reads to the evidence checkout. Its typed raw-event
  bundle must independently show exact successful `git rev-parse HEAD`, empty
  `git status --porcelain`, bulk kan, recovered suite, census, and CI invocations
  in that order. Census rows and CI JSON are parsed structurally. Comments,
  shell prefixes, wrong item types, malformed output, and output-only lookalikes
  do not certify a recheck. The generated bundle also binds the executing
  GitHub run. After the model exits, the workflow runner resolves the suite
  argv from the authenticated handoff, removes model-writable Cargo artifacts,
  independently executes that suite from a fresh target at the clean candidate,
  and retains a digest-addressed typed receipt. Every removal and
  wrong-candidate control is then derived from that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, single- and multiword
      Markdown proper-name speaker headings, and spaced/compact Unicode-dash
      variants. Ordinary
      Decision/Effect, Risk/Mitigation, Q1/Q2, AC1/AC2, and RFC source/scope
      summaries remain accepted. Malformed and ambiguous source requests
      likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned type-stable lifecycle, or a grader
      origin outside the exact repository/workflow-file/event tuple. The
      authenticated Codex home is the same isolated home used for every scenario.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully in order through
      root-owned executable-pinning wrappers from an independently rechecked
      clean checkout at the candidate SHA. Census and CI output is structurally
      exact; a post-session runner recheck discards mutable Cargo artifacts and
      independently executes the authenticated suite from a fresh target; and
      the generated bundle run ID plus workflow file/ref/SHA match the executing
      upstream run. All twelve registered removal/wrong-candidate controls are
      derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreifz772ednoznddial63lgdqaikgum6uigcu4fo7xsqgelgxgdhfs4",
  "sig": "71fb8abf2849eb1f752a1b501f72ddf6a73f6ed4f559c43c3b8eb9744cf73bce6dcf63e6a56f1a59c73522625acb99f7539727851aa0c03cd39f8eb8d5090d4f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtcyu42txa",
  "seq": 118,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1Mzk4MWY4MDEzM2JhYmZlYzUxMmJjMWFkYTE2YzQxYjY4ODBhN2JkaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUe0IGc7"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreighmhxlxqxq7ogyzw4swfyu5xpr6fsr3sm4nvxrpzkja2ufohhxze",
  "sig": "dd9db8d7a0ecafc51dbb43850ad9b435f1659f379ce59cf6fcca1ceb7555b29157209fd556faa0cb7b68ffdc87d03bf5de8875ce81f32fbd6f3f0c822172ae8e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreic3ttqzqsn4omku54cwu6vnqqqwlndkqotubog72yunjorjqfbkru"
  ],
  "rev": "223mtczwuxfdb",
  "seq": 119,
  "of": 125,
  "text_len": 591,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgW5zhmEm8cxVO8FanqthCFltGqDp0C439Yo1LopgUKo1mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg0YmY5ZTVjMmVkY2Y5NzA3NTQwOWEyZmVkODZkZWQxYmExMDRiN2EzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUf5rqy+"
}
---

adversarial review of v0.13-workflow-ergonomics at 4bf9e5c2edcf97075409a2fed86ded1ba104b7a3: BLOCK — the authoritative askme grader scans serialized claim envelopes without decoding event blocks, accepts any positive claim-count delta, and does not reconcile recording commands with raw Codex command events. An extra hand-written acquired-input block containing escaped multiword Markdown dialogue can therefore be certified. REQ-10/AC-11 remain unmet; decode and validate the exact newly appended event set, bind raw and wrapper commands, add a hostile serialized fixture, and re-review.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidmoslu2474cgkhfkkrvw6ol4hvvy3nwxwlfktktcmkf2dgod5ixu",
  "sig": "6c1b8050362c41cde286c99d1dbb705cdeb86a38e4b2adde45555df45a1a405a06e910c5f93e2c97fc705a13de1f16bfcc8393c3e9e9eb7e5012737ee027e146",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreic3ttqzqsn4omku54cwu6vnqqqwlndkqotubog72yunjorjqfbkru",
    "bafyreighmhxlxqxq7ogyzw4swfyu5xpr6fsr3sm4nvxrpzkja2ufohhxze"
  ],
  "rev": "223mtd25ych3s",
  "seq": 120,
  "of": 125,
  "text_len": 21772,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboNgqWCUAAXESIFuc4ZhJvHMVTvBWp6rYQhZbRqg6dAuN/WKNS6KYFCqN2CpYJQABcRIgx2Huu8Lw+42M25KxcU7d8fFlHcmcbW8X5UkGqFcc98lmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eCgxNDcyNzgzZTVmNWVkOTE5Yjg4ZDIwMDI4ZDM3NGM3ZTg5NTNkZDBkoWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KDE0NzI3ODNlNWY1ZWQ5MTliODhkMjAwMjhkMzc0YzdlODk1M2RkMGRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZSAfkMx4="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, single- or multiword Markdown
  proper-name speaker headings, and spaced or compact Unicode dash-dialogue
  forms. The approximation
  distinguishes speaker roles/proper names from ordinary structured labels
  such as Decision/Effect, Risk/Mitigation, Q1/Q2, and AC1/AC2; malformed or
  ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside `kan-tools/day` on `github.com`, from the
  exact registered workflow file/ref, workflow SHA, and `workflow_dispatch`
  event at that candidate and run. It requires closed, type-stable
  thread/turn/item lifecycle ordering
  and recomputes each outcome from addressed typed Codex events; changing a
  transcript while updating its digest must still fail the rubric when behavior
  changes. Raw recording-command events must equal the addressed wrapper log.
  Before/after kan snapshots are differenced by CID: non-recording paths append
  nothing, while explicit consent appends exactly one new claim whose
  `day-acquired-input` block decodes and passes the same schema validation as
  the public write boundary. Scanning a serialized envelope is not evidence
  about its escaped narrative fields.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session from the clean candidate checkout. Root-owned
  wrappers outside the model-writable checkout pin the real executables, reject
  the wrong working directory, HEAD, or tree state immediately before each
  command, and route kan reads to the evidence checkout. Its typed raw-event
  bundle must independently show exact successful `git rev-parse HEAD`, empty
  `git status --porcelain`, bulk kan, recovered suite, census, and CI invocations
  in that order. Census rows and CI JSON are parsed structurally. Comments,
  shell prefixes, wrong item types, malformed output, and output-only lookalikes
  do not certify a recheck. The generated bundle also binds the executing
  GitHub run. After the model exits, the workflow runner resolves the suite
  argv from the authenticated handoff, removes model-writable Cargo artifacts,
  independently executes that suite from a fresh target at the clean candidate,
  and retains a digest-addressed typed receipt. Every removal and
  wrong-candidate control is then derived from that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, single- and multiword
      Markdown proper-name speaker headings, and spaced/compact Unicode-dash
      variants. Ordinary
      Decision/Effect, Risk/Mitigation, Q1/Q2, AC1/AC2, and RFC source/scope
      summaries remain accepted. Malformed and ambiguous source requests
      likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned type-stable lifecycle, or a grader
      origin outside the exact repository/workflow-file/event tuple. The
      authenticated Codex home is the same isolated home used for every
      scenario. Raw and wrapper recording commands agree, snapshot CID
      difference is exactly zero or one as preregistered, and the sole consented
      append decodes as a valid acquired-input block; extra, malformed, or
      transcript-bearing serialized event claims fail.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully in order through
      root-owned executable-pinning wrappers from an independently rechecked
      clean checkout at the candidate SHA. Census and CI output is structurally
      exact; a post-session runner recheck discards mutable Cargo artifacts and
      independently executes the authenticated suite from a fresh target; and
      the generated bundle run ID plus workflow file/ref/SHA match the executing
      upstream run. All twelve registered removal/wrong-candidate controls are
      derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreibwkpza2pvyxcj3dsc5nxy37g6scnzkk6wncwfmnklqc22euweoka",
  "sig": "49869d69181aee74a30fce05a3bd421ba0067bb0575d9c0f2d5f5891330076ed25e50fd21ccce7d78e511a813c53c6b4ac9eb78656736f2144276859cb1cd33b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtd25z3o6l",
  "seq": 121,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgxNDcyNzgzZTVmNWVkOTE5Yjg4ZDIwMDI4ZDM3NGM3ZTg5NTNkZDBkaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUgH8NAi"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiabzofvtxsvqiw4vejusysidukoqcmpd46asplskwr6tb7ynbysra",
  "sig": "a256102c45f1889c0f32dadca989b983efd2cfd92eaf2c495e35d99591c493041b05e266154f9e40fc2ec87d753c2ec7df3834d20c9df63f27d9a3cad3f9d7d0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "decision",
  "cites": [
    "bafyreidmoslu2474cgkhfkkrvw6ol4hvvy3nwxwlfktktcmkf2dgod5ixu"
  ],
  "rev": "223mtd3ggubu2",
  "seq": 122,
  "of": 125,
  "text_len": 597,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgbHSXTXP8EZRyqVGtvOXw9a42217LKqapiYouhmcPqL1mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkMjIzNTI0YzZiZmUwMmQyZjk3YTljNzgwNTllYWJiNDQ5MGMzMWI2aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUhYzR7T"
}
---

adversarial review of v0.13-workflow-ergonomics at d223524c6bfe02d2f97a9c78059eabb4490c31b6: BLOCK — askme evidence proves one successful compound raw command, one argv-only wrapper log entry, and one valid new acquired-input block, but does not prove the recorder invocation succeeded or returned that CID. A failed day recorder followed by an unrelated manual append can pass. REQ-10/AC-11 remain unmet; record structured argv/exit/CID receipts, bind the returned CID to the sole new Observation envelope and its subject/author/citations, add a hostile compound-command fixture, and re-review.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie3noyz5iczxiwhkaajvqnza3htektnrcclwoe7svyodj75v23m64",
  "sig": "e796aa16d5204359c225d59b4a42df17437d34585744271619ebb505a9d43ace0aab366afc057d2d2e2d1046ded86cc2e6615f0bbe5134ce855385998880ef64",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "plan",
  "cites": [
    "bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua",
    "bafyreidmoslu2474cgkhfkkrvw6ol4hvvy3nwxwlfktktcmkf2dgod5ixu",
    "bafyreiabzofvtxsvqiw4vejusysidukoqcmpd46asplskwr6tb7ynbysra"
  ],
  "rev": "223mtd3mmipaj",
  "seq": 123,
  "of": 125,
  "text_len": 22182,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4PYKlglAAFxEiBItbtWgXF3JFzbq9+Un0BGnMzZd1NnXQtI+q8+6cYboNgqWCUAAXESIGx0l01z/BGUcqlRrbzl8PWuNtteyyqmqYmKLoZnD6i92CpYJQABcRIgAcuLWd5Vgi3KkTSWJIHRToCY8fPAk9clWj6Yf4aHEohmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgqFmQ29tbWl0eChkNjdmNDRjYmQ2NmRlMjJlY2E4ZWQ0MzYwODhjM2UyMjc3ODQwODNioWZGaWxlQXSCeCQuZGVzaWduL3YwLjEzLXdvcmtmbG93LWVyZ29ub21pY3MubWR4KGQ2N2Y0NGNiZDY2ZGUyMmVjYThlZDQzNjA4OGMzZTIyNzc4NDA4M2Jpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZSGUnVFk="
}
---

# Release: v0.13 workflow ergonomics, correction round 5

## Summary

`v0.13.0-beta.1` is a six-issue workflow-ergonomics candidate: #93, #143,
#152, #193, #195, and #204. It makes interrupted work cheaper to recover and
human direction more precise to record without turning day into a tracker or a
transcript. The release distinguishes post-compaction startup, retains the
newest bounded practice, gives handoff measurements immutable executable
coordinates, inventories visible handoff streams from one honest bulk read,
ships general `/askme`, and records acquired input and interventions only at an
explicit write boundary.

Accepted RFC 1 Result
`bafyreiciww5vnalro4sfzw5l36kj6qcgttgns52tm5oqwsh2v47otrq3ua`, addressing
`35c991c3b5949caf8ef1e8f71f9b6d47a1ae1ddf:rfcs/1-frame-indexed-process-model.md`,
is the normative semantic constraint. This Plan must cite that Result directly;
mentioning it in prose is not a graph edge. The candidate does not claim RFC 1
profile certification. In particular, #196 is deferred until kan exposes the
accepted RFC's principal, verification-method, capability, and view surfaces;
day will not standardize legacy roles as a substitute.

This correction supersedes correction-round-4 Plan
`bafyreibp7yfvp3iruqm2z5sbnwfywi4t2kgnnn5xctsmimxljozcf3flxy` and responds to
BLOCK verdict `bafyreicqpudqsj2gm5xfuw5lenkronehjet7nmr35zy3nfg5noz2l4wvfa`.
Round 4 correctly narrowed generic RFC work, but still called unavailable #196
a release blocker, relied on static handoff and `/askme` checks, admitted a raw
transcript through accepted fields, covered only part of the intervention
matrix, and described release machinery that did not exist. Round 5 makes the
candidate boundary equal the implementation and moves the unavailable or
noncritical remainder forward visibly.

The work serves `telos/legible-process`, `telos/honest-reads`,
`telos/affordance-not-enforcement`, `telos/v1.0`, and release target
`telos/v0.13-workflow-ergonomics`.

## Requirements

- REQ-1: The authoritative Plan is a published kan `Plan` claim that directly
  cites the accepted RFC 1 Result. It names the exact normative source, subject,
  committed `.design/v0.13-workflow-ergonomics.md` mirror, mirror commit, and
  mirror digest. The compatibility mirror supplies bytes to today's tooling;
  the Plan CID supplies identity and review selection.

- REQ-2: A repository-owned resolver must start from the selected Plan CID and
  recover its published claim, subject, RFC Result citation, source coordinate,
  exact commit/path, digest, and byte-identical mirror. Mutating any one of
  those fields must fail; no newest-file fallback is permitted.

- REQ-3: The typed candidate boundary is exactly issues 93, 143, 152, 193, 195,
  and 204. `xtask` owns the canonical set. `.release/v0.13.json` is an instance
  that must equal that set and its workflow, protocol, and publication-artifact
  sets exactly. The enclosing candidate SHA is supplied externally because a
  commit cannot honestly contain its own hash.

- REQ-4: `/handoff` records suite argv/full commit, census base/head/count, and
  CI provider/workflow/run/head/conclusion in a versioned
  `day-handoff-scopes` block. `day stream scopes` reads the newest visible
  handoff and emits only those coordinates without consulting current HEAD.
  `/wakeup` replays them. A moving-HEAD/merge fixture must keep byte-identical
  coordinates, and legacy unscoped prose must remain `UNCHECKABLE` (#152).

- REQ-5: `day stream list` derives visible live `agents/handoff/*` subjects
  from one bulk kan read and reports visible claim counts, bounded previews,
  and timestamps only when established. Withheld, unaccounted, missing, or
  failed published-read diagnostics make completeness explicit. CLI, MCP, and
  both list skills consume the same pure report (#204).

- REQ-6: `/askme` is a general non-atom affordance that establishes a topic,
  asks one adaptive question at a time, distinguishes supplied facts,
  decisions, unresolved items, and material effect, honors skip and stop, and
  asks explicit consent before recording. Declarable prompts remain #194
  (#193).

- REQ-7: Acquired-input and intervention writes are ordinary validated kan
  Observations. They preserve subject, actual signer, basis, material effect,
  and reported, separately authenticated, or signer-as-source provenance.
  All five fixed intervention kinds support the same-actor case. Multi-speaker
  transcript-shaped content is rejected inside every accepted narrative field,
  including compact colon, bracketed-speaker, single- or multiword Markdown
  proper-name speaker headings, and spaced or compact Unicode dash-dialogue
  forms. The approximation
  distinguishes speaker roles/proper names from ordinary structured labels
  such as Decision/Effect, Risk/Mitigation, Q1/Q2, and AC1/AC2; malformed or
  ambiguous requests append nothing (#193, #195).

- REQ-8: Practice projection keeps the newest eligible items under its cap
  (#143). Session-start treats `source=compact` as a distinct advisory
  reorientation that points back to the durable record, while ordinary or
  malformed input remains byte-identical to normal startup. No blocking
  `PreCompact` hook is registered (#93).

- REQ-9: Deterministic tests cover parsing, serialization, provenance,
  transcript refusal, non-mutation, bulk-read completeness, structured handoff
  scopes, compaction, practice order, and package parity. Every behavioral fix
  carries revert evidence. Static keyword checks describe prompt contracts only
  and never claim model behavior.

- REQ-10: `.release/protocols/askme-v1.json` preregisters decision, factual,
  unknown-topic, skip, early-stop, context-free, explicit-record, and decline
  paths. A real multi-turn Codex runner records raw JSONL, exact user and
  assistant turns, command observations, kan before/after reads, candidate,
  model, pinned harness version, GitHub run, and file digests. Authentication,
  skill installation, and execution share one isolated `CODEX_HOME`. The public
  grader is authoritative only inside `kan-tools/day` on `github.com`, from the
  exact registered workflow file/ref, workflow SHA, and `workflow_dispatch`
  event at that candidate and run. It requires closed, type-stable
  thread/turn/item lifecycle ordering
  and recomputes each outcome from addressed typed Codex events; changing a
  transcript while updating its digest must still fail the rubric when behavior
  changes. Raw recording-command events must equal addressed structured wrapper
  receipts carrying argv, exit status, stdout/stderr, and the returned CID.
  Before/after kan snapshots are differenced by CID: non-recording paths append
  nothing, while explicit consent appends exactly one new claim whose
  `day-acquired-input` block decodes and passes the same schema validation as
  the public write boundary. That CID must be the successful recorder receipt's
  returned CID, and its outer Observation subject, signer, and citations must
  agree with the payload. Scanning a serialized envelope is not evidence about
  its escaped narrative fields.

- REQ-11: `.release/protocols/reconstruction-v1.json` requires a genuine
  stream, acquired-input claim, non-manufactured qualifying intervention,
  scoped handoff, and fresh wakeup without the transcript. The evidence source
  is an immutable commit containing signed published kan claims and reviewed
  coordinates. One pinned project principal must sign an explicit review claim
  citing the acquired input, intervention, and handoff. The workflow itself
  launches the fresh Codex session from the clean candidate checkout. Root-owned
  wrappers outside the model-writable checkout pin the real executables, reject
  the wrong working directory, HEAD, or tree state immediately before each
  command, and route kan reads to the evidence checkout. Its typed raw-event
  bundle must independently show exact successful `git rev-parse HEAD`, empty
  `git status --porcelain`, bulk kan, recovered suite, census, and CI invocations
  in that order. Census rows and CI JSON are parsed structurally. Comments,
  shell prefixes, wrong item types, malformed output, and output-only lookalikes
  do not certify a recheck. The generated bundle also binds the executing
  GitHub run. After the model exits, the workflow runner resolves the suite
  argv from the authenticated handoff, removes model-writable Cargo artifacts,
  independently executes that suite from a fresh target at the clean candidate,
  and retains a digest-addressed typed receipt. Every removal and
  wrong-candidate control is then derived from that authenticated bundle.

- REQ-12: Candidate qualification requires the exact workflows named by the
  typed contract to succeed at the externally supplied candidate SHA. Askme raw
  evidence is uploaded and pushed to an immutable run-specific evidence ref;
  reconstruction accepts only a full immutable evidence commit, re-reads its
  published claims through kan under the pinned project review principal, and
  generates the fresh wakeup inside the candidate workflow. Missing, malformed,
  unreadable, wrong-SHA, pre-authored, or skipped evidence is not success.

- REQ-13: Candidate preparation updates Cargo, lockfile, plugin/install/docs,
  changelog, roadmap, and the generated current block corpus before the sole
  candidate commit. Existing publication automation must tag that exact clean
  candidate without a post-trial source commit. Post-publication verification
  binds release workflow, tag, crate, GitHub Release, and release claim to that
  candidate. This is a project release result, not a generic RFC certificate.

- REQ-14: #196, #194, #234, trigger-scoped practice, design-integrity and
  vocabulary work, and RFC rollout #227–#233 remain visible beyond v0.13.
  Changing milestone placement cannot change RFC 1's accepted bytes. #196
  remains blocked rather than falling back to legacy roles.

## Acceptance Criteria

- [ ] AC-1: (REQ-1, REQ-2) The published round-5 Plan directly cites accepted
      RFC 1 Result, and the resolver recovers its exact subject, source,
      commit/path, digest, and byte-identical mirror by Plan CID.

- [ ] AC-2: (REQ-1, REQ-2) Resolver mutations of CID, subject, RFC Result,
      source, commit, path, digest, or bytes each fail rather than selecting a
      different claim or file.

- [ ] AC-3: (REQ-3, REQ-12) `just verify-v013-contract` proves the committed
      manifest equals the typed issue/workflow/protocol/artifact contract.
      Removing or adding any member on either side fails.

- [ ] AC-4: (REQ-4) A structured handoff scope survives an actual branch
      advance and merge byte-identically; the paired legacy handoff reports
      `UNCHECKABLE` and explicitly refuses current defaults.

- [ ] AC-5: (REQ-5) Stream fixtures cover live/superseded/retracted/unrelated
      claims, timestamps, withheld and unaccounted subjects, missing and
      nonzero published-read diagnostics, and CLI/MCP parity through one fold.

- [ ] AC-6: (REQ-6, REQ-9) Static `/askme` tests cover the one-question,
      classification, skip, stop, and consent contract while scratch-log tests
      prove decline and early stop append nothing.

- [ ] AC-7: (REQ-7) Acquired-input fixtures cover reported, separately signed,
      and signer-as-provider sources. Intervention fixtures cover all five kinds
      for signer-as-source plus agent-reported and separately signed human
      sources. Provenance cannot be promoted by prose.

- [ ] AC-8: (REQ-7) A transcript smuggled through `--fact` or another accepted
      narrative field is rejected with an invariant-specific error and no
      append, including compact-colon, bracketed-speaker, single- and multiword
      Markdown proper-name speaker headings, and spaced/compact Unicode-dash
      variants. Ordinary
      Decision/Effect, Risk/Mitigation, Q1/Q2, AC1/AC2, and RFC source/scope
      summaries remain accepted. Malformed and ambiguous source requests
      likewise append nothing.

- [ ] AC-9: (REQ-8) Newest-practice retention and compact-versus-startup tests
      pass, malformed hook input equals ordinary startup, and plugin wiring has
      no `PreCompact` registration.

- [ ] AC-10: (REQ-9) Focused tests and strict lint pass, each implementation
      correction is demonstrated under revert, and the final full `just ci`
      plus commit census is green at one clean candidate.

- [ ] AC-11: (REQ-10) The preregistered real-harness workflow runs every exact
      scenario at the candidate SHA. The grader rejects missing scenarios,
      mismatched candidate/protocol/digests, multiple questions, nonadaptive
      follow-ups, continued questioning after stop, recording without explicit
      yes, mutation after decline, transcript content in durable claims, or a
      raw event stream outside the pinned type-stable lifecycle, or a grader
      origin outside the exact repository/workflow-file/event tuple. The
      authenticated Codex home is the same isolated home used for every
      scenario. Raw recording commands and structured wrapper receipts agree,
      snapshot CID difference is exactly zero or one as preregistered, and the
      sole consented append is the successful recorder's returned CID and
      decodes as a valid acquired-input Observation with matching subject,
      signer, and citations; failed recorder/compound-command substitutions,
      extra, malformed, or transcript-bearing serialized event claims fail.

- [ ] AC-12: (REQ-11) Reconstruction succeeds only when the fresh wakeup and
      bulk kan read contain the required stream, three CIDs, and every immutable
      suite/census/CI coordinate, and its raw events show the exact recovered
      verification commands completing successfully in order through
      root-owned executable-pinning wrappers from an independently rechecked
      clean checkout at the candidate SHA. Census and CI output is structurally
      exact; a post-session runner recheck discards mutable Cargo artifacts and
      independently executes the authenticated suite from a fresh target; and
      the generated bundle run ID plus workflow file/ref/SHA match the executing
      upstream run. All twelve registered removal/wrong-candidate controls are
      derived by the grader and fail.

- [ ] AC-13: (REQ-12, REQ-13) Every required workflow concludes success at the
      candidate. Publication creates no source commit and post-publication
      verification resolves workflow, tag, installed crate, GitHub Release,
      release claim, behavioral evidence, and reconstruction evidence to that
      same SHA.

- [ ] AC-14: (REQ-14) The roadmap and live issue tracker remove deferred work
      from the v0.13 milestone without closing it. #196 remains visibly blocked
      on kan RFC 1 rather than marked implemented.

## Architecture

### Claim-addressed Plan compatibility

The committed `.design` file remains the byte source current day validation
understands. A full-text kan Plan cites RFC 1 and carries an exact `FileAt`
artifact coordinate. `kan publish` makes the claim available in `.claims/`.
The v0.13 resolver is repository-owned release verification: it begins from an
explicit Plan CID, verifies the published envelope and citation, reads
`git show <commit>:<path>`, checks the digest, and compares the mirror bytes.
It never scans for the newest design.

`day design record --cites <cid>` also supports direct normative citations for
ordinary summarized design passes. The normative CID list appears in Plan text
identity so a repeat can distinguish an older uncited Plan from the governed
one.

### Honest handoff reads

`src/stream.rs` owns both inventory and the versioned scope block. Inventory is
a view-bounded fold. Scope rendering is deliberately independent of git HEAD:
it exposes the coordinates the handoff asserted so `/wakeup` can replay them.
This division avoids executing arbitrary command strings while still making
retargeting impossible; suite commands are stored as argv arrays.

### Explicit human-direction records

`skills/askme/SKILL.md` owns conversational policy. `src/events.rs` owns only
the explicit durable schemas and append boundary. The envelope signer is always
`recorded_by`; `Source::Recorder` must equal it, `Reported` authenticates no
third party, and `AuthenticatedClaim` derives its principal from a separately
signed visible claim. Transcript rejection is a schema invariant as well as a
CLI check, so hand-written blocks cannot bypass it.

### Candidate evidence planes

Deterministic Rust tests establish code behavior and non-mutation. The askme
workflow establishes model behavior through real multi-turn sessions. Its raw
evidence lives outside the candidate on a run-specific evidence ref. The
reconstruction workflow consumes an addressed reviewed real-work source commit,
verifies its signed kan overlay, and itself runs the transcript-free fresh
session that produces the graded bundle. Both use graders in `xtask`; neither
trusts a stored `passed` string or a pre-authored wakeup assertion.

Review verdicts remain Decision evidence on the reviewed subject, but the
shared vocabulary fold excludes the stable `adversarial review of …` form from
declaration and practice-content roles. A verdict therefore cannot replace the
telos statement that future sessions are supposed to evaluate.

The manifest contains only stable contract membership. Candidate SHA, workflow
run IDs, evidence commits, and publication coordinates are external evidence:
putting the candidate's own SHA inside its tree or an evidence commit's own SHA
inside itself would demand a cryptographic fixed point, not better provenance.

### Delivery order

1. Land deterministic feature fixes and their revert demonstrations.
2. Land the typed contract, protocols, real-harness runner, graders, workflows,
   and removal controls.
3. Correct and publish this Plan with a direct RFC 1 citation; commit and test
   the CID-rooted resolver.
4. Reconcile the live milestone, leaving the six candidate issues and moving
   the rest forward without closing them.
5. Finish version/docs/corpus preparation, run `just ci`, and commit the sole
   candidate.
6. Push the candidate and require every contract workflow at its exact SHA.
7. Run the real askme protocol, then genuine-work reconstruction when a
   qualifying intervention exists; do not manufacture one to clear the gate.
8. Run a fresh cold review. Fix and re-review any BLOCK or REDIRECT finding.
9. Publish the unchanged candidate and run fresh post-publication verification.

## Resolved Questions

- RQ-1: The v0.13 boundary is exactly #93, #143, #152, #193, #195, and #204.
- RQ-2: #196 moves forward because its accepted upstream surface is absent;
  legacy identity roles are not an RFC 1 implementation.
- RQ-3: Handoff scopes are a typed block rendered by `day stream scopes`; the
  read command does not execute arbitrary claim-provided commands.
- RQ-4: `/askme` remains a general driver affordance, not an atom or automatic
  recorder. Declarable questions remain #194.
- RQ-5: The recorder can be the authenticated provider/source because the
  claim envelope signs that statement; decoded blocks require equality with
  `recorded_by`.
- RQ-6: Transcript refusal applies inside accepted fields, not only to an
  imaginary `--transcript` flag.
- RQ-7: Real model behavior is evidenced by preregistered external bundles and
  derived graders; static skill scans claim only deterministic structure.
- RQ-8: The candidate manifest cannot contain its own commit SHA. The workflow
  invocation and resulting run/evidence claims bind external coordinates.
- RQ-9: v0.13 evidence is a repository-specific release result constrained by
  RFC 1, not a generic profile certificate.
- RQ-10: Process-aware retrieval #234 begins the next line; it may use
  structural eligibility followed by optional semantic ranking, but similarity
  will not become truth, trust, frame equality, or witness satisfaction.
- RQ-11: Explicit opt-in recording remains the durable half of `/askme`; a
  completed conversation alone records nothing.
- RQ-12: Deterministic tests and real-harness trials remain separate evidence
  planes and are labelled as such.
- RQ-13: Superseded by RQ-2: #196 waits for accepted RFC 1 implementation and
  no compatibility fallback is a release feature.
- RQ-14: Stream completeness still includes withheld, unaccounted, missing,
  and failed published-read diagnostics.
- RQ-15: Intervention claims still preserve actual signer and distinguish
  reported from authenticated sources; signer-as-source is the added case.
- RQ-16: Fixed `/askme` prompting and intervention kinds ship; declarable
  variants remain behind shared vocabulary/preferences work.
- RQ-17: Accepted RFC 1 Result and its exact source remain normative even where
  implementation obligations are deferred.
- RQ-18: v0.13 still does not claim generic profile-v1 realization or
  certification.
- RQ-19: Release, trial, reconstruction, grading, and verification remain
  repository-owned rather than public day verbs.
- RQ-20: `.release/v0.13.json` remains an instance of the independently typed
  contract, not its own authority.
- RQ-21: #227–#233 remain the seven explicit accepted-RFC rollout records;
  milestone movement cannot revise accepted semantics.
- RQ-22: The v1.0 non-author/third-project bar remains the deliberate deeper
  semantics pass after the v0.13 ergonomics boundary.

## Open Questions

None.

## Out of Scope

- Implementing kan RFC 1 or legacy-role approximations for #196.
- Generic RFC 1 profile declarations, realizations, or certificates.
- Process-aware embedding retrieval and progressive disclosure (#234).
- Declarable `/askme` prompts (#194) or project-defined intervention kinds.
- Automatic acquired-input/intervention recording or raw transcripts in kan.
- Trigger-scoped practice, design-integrity, vocabulary-pack, and #227–#233
  implementation work.
- Calling a candidate released before real-harness, reconstruction,
  cold-review, and post-publication coordinates are actually available.

***8<***
---
{
  "v": 3,
  "cid": "bafyreihapga4eu6rbdf5mupjthiclggmts7xcweon5jx7v3s7dmmq6on6e",
  "sig": "25bcaa9be97b2ec4da7e7f6ba61bf09656fae3ec90cadaf79702e9008593de336cfe288b5a64219bcb33aa72ffc7ba6348c7c59ced3b674d66322145633dcec5",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "v0.13-workflow-ergonomics"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtd3mnbcxp",
  "seq": 124,
  "of": 125,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GXYwLjEzLXdvcmtmbG93LWVyZ29ub21pY3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNjdmNDRjYmQ2NmRlMjJlY2E4ZWQ0MzYwODhjM2UyMjc3ODQwODNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWUhlM6NK"
}
---
