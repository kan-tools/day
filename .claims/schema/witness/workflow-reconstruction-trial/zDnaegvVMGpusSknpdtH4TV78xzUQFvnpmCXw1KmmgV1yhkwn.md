---
{
  "v": 3,
  "cid": "bafyreic5yl7ejhox45va4ea3e663cply7gzmp2uie4xqmphfsvsus4xo4m",
  "sig": "f3a1eeaf978488baa58a3b444911f7dbc917dc45855b05970f08abe7e84e891f6338f2ea8c21708798c5697424db9bf8e742263013d32401108e2024f0a46b43",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "schema/witness/workflow-reconstruction-trial"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mt3ek35xzn",
  "seq": 0,
  "of": 3,
  "text_len": 488,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2FseCxzY2hlbWEvd2l0bmVzcy93b3JrZmxvdy1yZWNvbnN0cnVjdGlvbi10cmlhbGlhcnRpZmFjdHOBoWZDb21taXR4KDVkNmMwY2M4Zjg3MTA5ZTI1MWI5MTNiYzdmMmFhOGQzZTdmY2JlM2Zpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZCqAR94E="
}
---

The v0.13 workflow reconstruction trial probe, established by the recorded witness interview. It remains absent until a preregistered protocol-v1 Result carries the exact anchored pass marker; failed trials, prose that merely mentions the marker, other trial subjects, and later protocol versions do not satisfy it.

```day-witness
{"claim":{"kind":"Result","subject":"trial/v0.13-workflow-reconstruction","starts_with":"workflow reconstruction trial PASSED (protocol v1, rc v0.13)"}}
```
***8<***
---
{
  "v": 3,
  "cid": "bafyreihairhjwt26vt6brmpi5plj2cmvxitthza347hdenvuc4h4sqwx3q",
  "sig": "c766b8f5dad58bace54674e58c5207971d448c454e5741d9d61e85b748a7c9d86635ae56b2543814ba7fb5a675ed7dc2aa42b71937ba31b251b7effddb39c468",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "schema/witness/workflow-reconstruction-trial"
  },
  "kind": "observation",
  "cites": [
    "bafyreicfzosprhmptdyuiw5fo2fti7zp3c4edlispgo3cmcpl7wr363zsm"
  ],
  "rev": "223mt5f2dz2ht",
  "seq": 1,
  "of": 3,
  "text_len": 584,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgRcuk+J2PmPFEW6V2izR/L9i4Qa0SeZ2xME9f7R37eZNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4LHNjaGVtYS93aXRuZXNzL3dvcmtmbG93LXJlY29uc3RydWN0aW9uLXRyaWFsaWFydGlmYWN0c4GhZkNvbW1pdHgoYTU1YjIwOThiMjdjNGRmM2Q2YjI3MGQ1MmIxYjM2Mzk3MmU3ZjJlMGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkawJ+BTQ=="
}
---

Correction-round witness probe. Supersedes the prefix-only Result probe bafyreic5yl7ejhox45va4ea3e663cply7gzmp2uie4xqmphfsvsus4xo4m, which could accept prose beginning with a pass marker without checking a run. The command reads the structured day-trial Result, fetches its immutable evidence commit, verifies candidate and protocol coordinates, hashes every manifest entry, recomputes scenario and negative-control outcomes, and fails on missing, malformed, mismatched, failed, or uncheckable evidence.

```day-witness
{"command":"day trial verify v0.13-workflow-reconstruction"}
```
***8<***
---
{
  "v": 3,
  "cid": "bafyreibwr5nk42qes6shejluzv6bmffuhpgvknejj6hobpba5tv7mqvgxy",
  "sig": "f42f268ec01947d44373bcc3111697a06b99d26aa839c3debe2efb781b1899885ec8b7318473bd902144bb6c3431a2754cc25dc0f69a34b7bccf5a1024261fd9",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "schema/witness/workflow-reconstruction-trial"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt5f2ffzt6",
  "seq": 2,
  "of": 3,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWx4LHNjaGVtYS93aXRuZXNzL3dvcmtmbG93LXJlY29uc3RydWN0aW9uLXRyaWFsaWFydGlmYWN0c4GhZkNvbW1pdHgoYTU1YjIwOThiMjdjNGRmM2Q2YjI3MGQ1MmIxYjM2Mzk3MmU3ZjJlMGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkawLX+vw=="
}
---
