---
{
  "v": 3,
  "cid": "bafyreicphqsjq3a647ebsrleworxtw3xnomyrb2yqecwtuumqolqkuolxi",
  "sig": "d4aedd0cda04a0a9e109c319bcf19e7030a9631cf9be4a8027c71f199fd1ca135112493d8aa7c60edbe2f3974323589777124c1ce6e2a2c31d74e38c4e2dbeac",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mt5yfjcn2h",
  "seq": 0,
  "of": 69,
  "text_len": 195,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCgzZjlhZTM5Y2M3N2MzYjQxMDlhMzg2YzE0M2Q5NjY1MjNhMDhiYjg0aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWR+W9Eul"
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 13 check(s), 0 failed, 4 warning(s), 0 unchecked, 0 open question(s) [doc 15615:c63b99ba3a7e4fc9]
***8<***
---
{
  "v": 3,
  "cid": "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq",
  "sig": "4624e5ba83f53e7b317d3be0ea9b8e7131fc4f5074b23e352a578633fce7b11105dc1a0b6d6f9caf58b47ebef127003d7e950cd948b188aa4b8773187b367e86",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreicphqsjq3a647ebsrleworxtw3xnomyrb2yqecwtuumqolqkuolxi"
  ],
  "rev": "223mt5yfjqsj7",
  "seq": 1,
  "of": 69,
  "text_len": 597,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBPPCSYbB7nyBlFZLOjedt3a5mIh1iBBWnSjIOXBVHLumZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHNkYXktcmZjLWZvdW5kYXRpb25zaWFydGlmYWN0c4GhZkNvbW1pdHgoM2Y5YWUzOWNjNzdjM2I0MTA5YTM4NmMxNDNkOTY2NTIzYTA4YmI4NGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkflvtheQ=="
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 13 check(s), 0 failed, 4 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidb2h424mplg6varmt5ndjs5u36ejpp4wjxxan22fdukc7hbydet4",
  "sig": "813b9b0f762f051ccdaac1c9507ab5c1e79ad78049afa3d70198440118a1ff0e4881521d065663280499353177f1c25eaf3c4ca0af99e14a6054520341bc4ed4",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt5yfk2lwv",
  "seq": 2,
  "of": 69,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg8RmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9ubHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHNkYXktcmZjLWZvdW5kYXRpb25zaWFydGlmYWN0c4GhZkNvbW1pdHgoM2Y5YWUzOWNjNzdjM2I0MTA5YTM4NmMxNDNkOTY2NTIzYTA4YmI4NGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkflwBHLA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidhlew7af67dhbwih46hjusexj4tsgekeox6sbfxckbptap7p23bm",
  "sig": "a8b4a90381872425dbcf251aca29a01ec9ea499675ea0980ec8d5bd79877d0fa16c9ab1945065f7b92d013e0222b720c5d594daf5a0976c89d26d9e18fb37f8e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfks36j",
  "seq": 3,
  "of": 69,
  "text_len": 186,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5cMBCc="
}
---

RQ-1: Day adopts separate RFC and ADR disciplines modeled on kan RFC 0; working designs, forward-looking public contracts, and decisions actually taken retain different truth conditions.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiepye4tcvp7lhglg643r2c4p3pm77ijwihqp4e44uqotoyc54sswq",
  "sig": "eeeaeff26b1e228264e905efe3222a98f235032f06335024cc0e47606dc3209c5ab24cf73cdc0235a75f4d91d5fb11d16879d874a8fb7274afcfd13768e7d810",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yflac33",
  "seq": 4,
  "of": 69,
  "text_len": 142,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5cTH7U="
}
---

RQ-2: RFC 1 defines the ontology and operational approximation but defers concrete executor kinds, dynamic CLI projection, and pack transport.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihs45tg3aeek2gyusf3thwattbqfkc3jvkdn46zepk23e724lhmhq",
  "sig": "3968ade9d5922e4e1a2fa689372748d3f30dd782ee0a1d80067139a420ed8f2749b8307a9a54d6a947cbef2d1c6d2fcdf34525e113241f833cdcaae2faedad16",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfloi2s",
  "seq": 5,
  "of": 69,
  "text_len": 171,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5caN60="
}
---

RQ-3: Frames are normative dependent contexts in RFC 1; operational profile v1 has one implicit local frame and must not present its assessments as globally settled truth.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihyoai2trids4sbsmptkpx7jq44jrdl5xbg6x35hxrpa2mffsaakm",
  "sig": "dc848d7df3ef64881bfbbc3e49ad951d8b77b5a4c5192683bea997146114761f2e65f0a6c2de63b7a3b506d55b5a1bbd27fcf847086372a691fc494033931f1b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfm4p7m",
  "seq": 6,
  "of": 69,
  "text_len": 136,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5chVEA="
}
---

RQ-4: Artifacts, evidence, assessments, certificates, witnesses, and probes are distinct objects with the relationships stated in REQ-8.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicmwsnn3kxz52bdcaa6l5m7wp3zkesyvmdjxzz4dmnwjfvamkxjby",
  "sig": "6d69d2f27906c782821ad21219baaaa81c3b76bf8b60fbb82baac08069356a517cf2207c69dbb32b238025bc74ee6cde1fc0e5b5422920b1b15547b64956ff3f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfmktzn",
  "seq": 7,
  "of": 69,
  "text_len": 176,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5coZ4k="
}
---

RQ-5: The category-theoretic model is an explicit aspirational target with a convergence trajectory, not an implementation claim or a prerequisite that the full formalism ship.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicdjvywgwkcrc5bbjt57cwhwxpg36p23qwwi3t6gtmvflcjbxwk3y",
  "sig": "f1e9a3fbfc58b10e737183d32ec6b0158c2ad1dd6aaf4d6777207703d5565a490bab4fb047332aab42c2ba8a5a5e01fee4c3379086b52c98ac5902dc4ddfc8e2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfmz66o",
  "seq": 8,
  "of": 69,
  "text_len": 108,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5cvkCs="
}
---

RQ-6: RFC 1 acceptance precedes further v0.13 feature implementation. The full denotational target does not.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihbm2qfx7ysdkocydtufaterjarbojomq2ypebxmjopiv2phnvrgm",
  "sig": "d2af2ff0d77ca3ad7326b77681d8bcbb46bb084ce55293cf06ea9cbe8a11b7937eb9d7ee7ba35591e22d591522ece3ecd1579025efbdd3e83da74f05050d6164",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfnhi6k",
  "seq": 9,
  "of": 69,
  "text_len": 169,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5c2uCA="
}
---

RQ-7: Rollout details are scoped through GitHub issues derived from RFC 1; they are not baked into the foundational RFC merely because the current repository needs them.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifcn4la5xgurqbpcoyommhcn7hw246s7aqitowu2njf46zpbxafn4",
  "sig": "1b5f927d1a198ef08f95f1cf41acce9bff782ce4dac2b070da8bfad2922fe9be0629aa857ab971af0102bc546abf16b27c42a3f55246290af6dcca0997eac7de",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfnvolz",
  "seq": 10,
  "of": 69,
  "text_len": 124,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5c90dE="
}
---

RQ-8: Markdown-with-LaTeX is the canonical exploratory mathematical document; rendered HTML is a derivative reading surface.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaafsq7dnugfdc5qyldwlrylug6ms2moz7rnolsrj5otj5gdhpxqq",
  "sig": "fd8dcfacfc9ce6b599d721034542b3793bb75689808c0632eb61dc3f6a7a34eb41fe2743cd2a624f8de640782e38eb80978a6129c7d6115a9b98acbcb72365a6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yfodzsw",
  "seq": 11,
  "of": 69,
  "text_len": 273,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgiD8VbTskaeVlm1jDuo9DTZOuJTGS1FZLeccWUwvMMiRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNmOWFlMzljYzc3YzNiNDEwOWEzODZjMTQzZDk2NjUyM2EwOGJiODRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH5dE/qk="
}
---

RQ-9: Legacy `day-telos` witness lists default to `sufficient`. This permits a coherent certificate to support the telos but renders its absence as `not certified`, never as proof that the telos is false. `Necessary` and `exact` relationships require explicit declarations.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiemtk77axqq33b35oowzxkjmysjk6vw7gh3m4ylqglu2sn3a6hjoi",
  "sig": "4ef660f8ccaad1fef3d3fc07742caa033512dd65baf90896e0784ce66127d5791f36c76c1334208befb8ee707a1216d773a3587d186ed3f41aac31f0d2ba8c0f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreicphqsjq3a647ebsrleworxtw3xnomyrb2yqecwtuumqolqkuolxi"
  ],
  "rev": "223mt5yuvteup",
  "seq": 12,
  "of": 69,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgTzwkmGwe58gZRWSzo3nbd2uZiIdYgQVp0oyDlwVRy7pmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDJjZGEzYTliYTNlOWJkN2IxZjQxMTQ1MWIzZjU0NTkwZjZhYTAwYjFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH7W8qto="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 15615:c63b99ba3a7e4fc9]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigt7m5jkgpbcrttjfyhfbw5m6zfmwmbqkinp7dlvrdguntbsnu5ee",
  "sig": "eed811d83f7ae286174395e7657e54e240bcf2c25bf3432dbc1c6c8c17330638134ca5576aa887e0b7b2b209b829f243fe354c0d972dd30e7e51c395feae3a4f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreiemtk77axqq33b35oowzxkjmysjk6vw7gh3m4ylqglu2sn3a6hjoi",
    "bafyreieih4kw2ozenhswlg2yyo5i6q2nsoxckmms2rlew6ohczjqxtbseq"
  ],
  "rev": "223mt5yuwchhz",
  "seq": 13,
  "of": 69,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiCMmr/wXhDew7651s3UlmJJV6tvmPtnMLgZdNSbsHjpctgqWCUAAXESIIg/FW07JGnlZZtYw7qPQ02TriUxktRWS3nHFlMLzDIkZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCgyY2RhM2E5YmEzZTliZDdiMWY0MTE0NTFiM2Y1NDU5MGY2YWEwMGIxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWR+1xDVW"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidfmln75u5myqgkjpeyqjloms2wxfv7xlxkoezqzqvriloximqk4u",
  "sig": "dfc7e7028f41978b688dd4e487eb5cbeb96907f04964691c6602039cb39004a362fe7860e241ffa51f934430da2f5e43104c5466406cd433e251652221163e3b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt5yuwmqql",
  "seq": 14,
  "of": 69,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg8RmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9ubHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHNkYXktcmZjLWZvdW5kYXRpb25zaWFydGlmYWN0c4GhZkNvbW1pdHgoMmNkYTNhOWJhM2U5YmQ3YjFmNDExNDUxYjNmNTQ1OTBmNmFhMDBiMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkftclaag=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiefda4ahnu3ofnpbfijfhsqw3g3wknowkilhkwvkgirxin4vd64ua",
  "sig": "22687e3aaaf51167793f630c13b1bd2cdd6151e7d07514d8ff201cca5a40216f0eac88b703e1537d42937a3f2fb7a9d90b9e3cae04e4df833bc793b359a305ce",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt5yv23y7f",
  "seq": 15,
  "of": 69,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDJjZGEzYTliYTNlOWJkN2IxZjQxMTQ1MWIzZjU0NTkwZjZhYTAwYjFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH7YA+Ds="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci",
  "sig": "1f42cdd11b714af1db532294fcd2bf35259a1c8fb13827fccae94defdeca6efa5d15899bb61b8d11a29fe8d3672186c9d9faa52a87d8f14c8324a604d8657a0e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreigt7m5jkgpbcrttjfyhfbw5m6zfmwmbqkinp7dlvrdguntbsnu5ee"
  ],
  "rev": "223mt62nbmsxx",
  "seq": 16,
  "of": 69,
  "text_len": 248,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg0/s6lRnhFGc0lwcobdZ7JWWYGCkNf8a6xGajZhk2nSFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICZ5Yeg="
}
---

adversarial review of day-rfc-foundations: BLOCK — RFC 0 publication is recursively inconsistent, RFC 1 realization cells are ill-typed, profile v1 defers mandatory encodings and outcomes, and the validator certifies structures it does not check.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiainoag72mgjx5vmaw3gine6ymzocz2o5nqzwyjanpuz23czxnjpm",
  "sig": "8742bb2ea9842d21c37723e4321a34c606809f8dcdf440a132db3e10be7b12a845731ca33b2cb445493d108597d55361ae2bf0bd581ec6b8225393186292f622",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62nrcn75",
  "seq": 17,
  "of": 69,
  "text_len": 397,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICd0TDM="
}
---

RFC 0's publication protocol is recursively inconsistent: normative byte identity includes the Kan-claim metadata, but publishing commit A and inserting its CID creates different normative bytes at commit B. Remove the backlink from normative bytes, store it in a derived non-normative index, or exclude publication metadata from normative equivalence; add a fresh-clone FileAt resolution fixture.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicdcuenffsoygohacafdl2ui4mzknmu3io5benc4zz4houkz6gs4u",
  "sig": "5a5dcea48ddd12e48a88cd15bfd16cdf189ed8d2c46751ac86ca74ec23fc26745eb620ead8061691cca6cf98dd41d9c21a3bf60fe1ec651e3e8ab90d384bc8bd",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62nrsc7b",
  "seq": 18,
  "of": 69,
  "text_len": 423,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICd8IDI="
}
---

The RFC/ADR validator reports templates and indexes valid while hostile mutations removing template sections, ADR metadata, reverse index entries/status agreement, accepted-state review metadata, and historical-number identity all survive. Validate templates structurally, ADR metadata, bidirectional exact index coverage, file/index status agreement, number identity, and accepted-state metadata; self-test each guarantee.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigbh2xo52gspee3rjpzb5a3a6or2ky44cbgv2fyoc6zclyfqq6pqm",
  "sig": "ad13af56580864a73abf574b91115cacb5f042bebc6c70d6b18e565932b1892d4c065e368b47192030eff80ec201a42514cc7753fb54112c8332c3dc52714ea4",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62nsbxz2",
  "seq": 19,
  "of": 69,
  "text_len": 307,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICeD924="
}
---

RFC 1's realization cells are ill-typed under either composition convention: the bridge is written right-to-left as A_n ⊙ ... ⊙ A_1, but targets use B ⊙ T and A_i ⊙ P_i. Choose and declare one convention, repair the design/RFC/sketch consistently, and add mechanically type-checked boundary vectors.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiboidn6q44mt6kokcmkts6nfq3tjl6gbdkquw2cot723zemyiqnim",
  "sig": "32ecb64f5946231d786f610f12e6ef256ddade16dacf6dbc904fba5eb68485636dd268786164be1cc7ea9e3369ab0a3cb0aef2ba46baa0d494ac943b8633c281",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62nsrsvi",
  "seq": 20,
  "of": 69,
  "text_len": 449,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICeL4wE="
}
---

RFC 1 does not coherently connect its equipment, predicate, and witness semantics: teloi change from presheaves to proarrows without an embedding, Obs and the soundness transformation lack a common defined functor category, and an equipment-valued pseudofunctor is called equivalent to an ordinary fibration without the required double-categorical construction. Instantiate the structure or weaken these equations to candidate intuition until typed.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic64e5ssmp3qodhxarrpr2dqprc2omy4zexvbjxvisypjjh75xijq",
  "sig": "251afb52621b891c0c2051bdcc44b6a69f8fe80e13b0a1ee9de039fdef19467553c3116195cc63002afeb76869fd7073a61db80f1a55bbe8dda51cb48b7f2428",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62ntbmbq",
  "seq": 21,
  "of": 69,
  "text_len": 406,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICeTyC4="
}
---

Operational profile v1 is not independently implementable: mandatory shared-coordinate correspondence and assessment certificate fields have no encoding; necessary/exact relationships are both required and deferred; the result vocabulary lacks a sound refutation state; and reference vectors are only a checklist. Specify versioned shapes, parsing, compatibility, result conditions, and executable vectors.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicajy6ycbocdbb7zn6bzilxnbkf2zi3owkuwemkipftee3cnungr4",
  "sig": "a66878e8feb6cdd2dabc1315b778cbe5609a679713289a6694211fa5737129607b9ff31da2302e23ab37bb6a8a220064ecc52f7f75a395b01b0446998538543e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62ntrdnp",
  "seq": 22,
  "of": 69,
  "text_len": 304,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICebpgg="
}
---

Frame migration is asserted without the required base-change squares, mate transformations, directions, or vectors. State the precise pullback/base-change diagrams and strong/lax comparison maps, and supply lossless, lossy, unsupported-procedure, successful/failed gluing, and incomparable-frame vectors.
***8<***
---
{
  "v": 3,
  "cid": "bafyreib5s5riewekrnlvriidr3vykyy5n4lgugqm7mnm42ftmakg6zsn6i",
  "sig": "7a7552dee1fb7202ea297805c141c6bb320340b6f52f25697f80c7ca1cd1d1a766ae33d76e3d7c564fb4bda91d39e60c26229ca9c5f7351da61aeb28c86b7b6a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62nub3qu",
  "seq": 23,
  "of": 69,
  "text_len": 348,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICejhmU="
}
---

The conformance census is incomplete: the design lists thirteen terms while AC-4 says fourteen, and the RFC table omits Vocabulary, Pack, Probe, Evidence Context, external Artifact, exhaustive block/field/outcome coverage, and explicit day assess atom mapping. Reconcile the vocabulary and provide one row per required term and operational surface.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihgett74eutppuvr3pivyuj4puluc5qltjfmy4nbduhlafszsklsy",
  "sig": "32a173a3ae7bebf811aa4ee7f82199cf7f654b28cad514231eae5df2e0b9a6fb199be51c1c025f6412befa9e1bc44310f012434865937a021fc622ed44c83830",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreickogt7nsnfoqqutdfz5qknfgsdwlyfhwmmp3salyl5ricde2zeci"
  ],
  "rev": "223mt62nuqz6u",
  "seq": 24,
  "of": 69,
  "text_len": 368,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSnGn9smldCFJjLnsFNKaQ7LwU9mMfuQF4X2KBDJrJBJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzYTE5NDc5NDgzNzk0NTlmZjNhZjBiNjdiN2Y4NzU0YzdmZDg5MzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZICerfCE="
}
---

Rollout and self-application remain unfulfilled: no RFC review PR or publication claim exists, rollout issues do not cite RFC 1, the roadmap and canonical v0.13 design do not cite it, and the v0.13 design retains day trial verify. After semantic fixes and renewed review, perform the accepted lifecycle, publish exact artifacts, create scoped issues, and revise v0.13.
***8<***
---
{
  "v": 3,
  "cid": "bafyreif77pxo3nd3feblp3v4rzh5nzpeckundyq46rsnpnb6d32ehhsjhe",
  "sig": "8ffb91d0ace7c0c557519532bcfc4e47bedf432211fd837215248ad067f1c00f15a2a432c063d69bc6524b53a40e15949562b9ac82d6f5941d8c75fd675ffb28",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt62nzsabn",
  "seq": 25,
  "of": 69,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCgyM2ExOTQ3OTQ4Mzc5NDU5ZmYzYWYwYjY3YjdmODc1NGM3ZmQ4OTMzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSAn/Bh/"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiayetzvurbljt7f32vlgkr4ako4my2cyta4uuus2pkebr73wdi3va",
  "sig": "3c0abb8b39f5fac407a6c0e5320cd7aa59ec27987da4a4c47f0a6b74382d73477c20967843311a897bfd397dfd351da9f9ce8dc080706eb0b876797aef2c4dea",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiemtk77axqq33b35oowzxkjmysjk6vw7gh3m4ylqglu2sn3a6hjoi"
  ],
  "rev": "223mt66r6ahu7",
  "seq": 26,
  "of": 69,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgjJq/8F4Q3sO+udbN1JZiSVerb5j7ZzC4GXTUm7B46XJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDc3NWEyZDE2YmZiMzcwMmJmYTE3ZDJkY2M0OTAxYzJkNWE0M2YwYTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIS5DNts="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 18793:73e022bd2e2c7a78]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua",
  "sig": "935688668a698a9e0bfd0eb632821ca38deb590f14ef5b5db796555ef0edb43e1c1150d3e8922c1a22617d8434ff09812ab9cccb978681503b9341e941321ff2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreiayetzvurbljt7f32vlgkr4ako4my2cyta4uuus2pkebr73wdi3va",
    "bafyreigt7m5jkgpbcrttjfyhfbw5m6zfmwmbqkinp7dlvrdguntbsnu5ee"
  ],
  "rev": "223mt66r6poj7",
  "seq": 27,
  "of": 69,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiAYJPNaRCtM/l3qqzKjwCncZjQsTBylKS09RAx/uw0bqNgqWCUAAXESINP7OpUZ4RRnNJcHKG3WeyVlmBgpDX/GusRmo2YZNp0hZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NzVhMmQxNmJmYjM3MDJiZmExN2QyZGNjNDkwMWMyZDVhNDNmMGEzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSEuStF6"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreid7odfelqfjsvg7nnufla7weqe675ofdw7s3trhpl4h7hkfl4onu4",
  "sig": "6ee36fc45b58edd1dcf202d036f7b37a18d9cf5d234fca16de735dfb9f490c0d2d7064c1b8a307cf605c91ec8ac684ba35a383065d83e0032235de4a28715bc8",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt66r72bfh",
  "seq": 28,
  "of": 69,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDFsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3NzVhMmQxNmJmYjM3MDJiZmExN2QyZGNjNDkwMWMyZDVhNDNmMGEzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSEuUB0F"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihrwbewfmcyjyg6ora6stostfdd4nzlawfqxuplx7ofbhwijlfghe",
  "sig": "d1988671b69108166d8e12172b61e5351cb4e284d8025e5504af1a1934be7a1722d029f08c9a819cb89185cfcc891b4a659d264b54f43ac322c704ed17e6f254",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua"
  ],
  "rev": "223mt66r7slve",
  "seq": 29,
  "of": 69,
  "text_len": 230,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgEM2ibr83AZ8hi6vi7u4woIKY4gmrCPurfn0EJoi5QKBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDc3NWEyZDE2YmZiMzcwMmJmYTE3ZDJkY2M0OTAxYzJkNWE0M2YwYTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIS5cRv4="
}
---

RQ-10: Operational profile v1 supports an explicit versioned `sufficient` relationship. Legacy lists remain flat component reports, while `necessary`, `exact`, and sound refutation wait for a later profile with defined algorithms.
***8<***
---
{
  "v": 3,
  "cid": "bafyreih4sw43yrjglj7ecw6uhmwvemjexzh2ht3tn27r3jyxl7aqghzzmu",
  "sig": "385a479e072ee63421657f565397dd6ade127f91b98331521955be94b8acbdc5792a0014b79c1d1b0c2793a1a3d6a5980fb26cd1f30418bc466c4450bcae3f4e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua"
  ],
  "rev": "223mt66rabsii",
  "seq": 30,
  "of": 69,
  "text_len": 189,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgEM2ibr83AZ8hi6vi7u4woIKY4gmrCPurfn0EJoi5QKBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDc3NWEyZDE2YmZiMzcwMmJmYTE3ZDJkY2M0OTAxYzJkNWE0M2YwYTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIS5j4V4="
}
---

RQ-11: Composition is conventional right-to-left. A bridge $B=A_n\odot\cdots\odot A_1$ realizes $T$ through $\eta:P_0\Rightarrow T\odot B$ and local cells $P_{i-1}\Rightarrow P_i\odot A_i$.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiesupeo4xuwykcwpbahegljlhzu363tcifzb63eibmqypv7folt2y",
  "sig": "da5c1462788bfa3c8cb3d353c39b358b7c488d50e0d2c3678269730f349bc6312c5b07305868a0ebc5477f49fb375a6f7f95dab176a448a371b6673846e1db2b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua"
  ],
  "rev": "223mt66rar537",
  "seq": 31,
  "of": 69,
  "text_len": 122,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgEM2ibr83AZ8hi6vi7u4woIKY4gmrCPurfn0EJoi5QKBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDc3NWEyZDE2YmZiMzcwMmJmYTE3ZDJkY2M0OTAxYzJkNWE0M2YwYTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIS5ri7w="
}
---

RQ-12: RFC claim discovery is non-normative and external to RFC bytes; no RFC embeds the CID of a claim addressing itself.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidochjwvjkoclgjqrek3khxvnoq6aimkekddcmjte75lo7zyii6ba",
  "sig": "145f83a3fb1b73f8eec920e5505efdd3eb0aa1bdeb56df3baf8c0b449c1d99ae48f1412f513d8f79c88db349bc41e7aaf799778b241513b685cec378e3cd6735",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua"
  ],
  "rev": "223mt66rbakdi",
  "seq": 32,
  "of": 69,
  "text_len": 209,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgEM2ibr83AZ8hi6vi7u4woIKY4gmrCPurfn0EJoi5QKBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDc3NWEyZDE2YmZiMzcwMmJmYTE3ZDJkY2M0OTAxYzJkNWE0M2YwYTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIS5zQMI="
}
---

RQ-13: Draft readiness, formal review and acceptance, and post-acceptance rollout are separate delivery stages. AC-12 through AC-14 are lifecycle gates, not claims that a Draft build has already been accepted.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicdrj63ygfpi4zlgszil2uqtl36sda6xo6j5di2di4jcz2an7ge3i",
  "sig": "c80c2ac271eea14cbd76c762e9ed9bc9a0a8eb41bfeb2c6397818cee913d2d225abe88558edf7f96b47c5209d7057898121bd1782e3e9974611d8298bef916a4",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt66rmhlmt",
  "seq": 33,
  "of": 69,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDc3NWEyZDE2YmZiMzcwMmJmYTE3ZDJkY2M0OTAxYzJkNWE0M2YwYTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIS8mxfA="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze",
  "sig": "53a7f74103072517fd8460e228cdc26f40d8212fab6dc8b31ff2656e33984571240108ffa6d46af66e482f0eecace0100eb95fb24436d08727c9e01cfe6043b9",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua"
  ],
  "rev": "223mt67h7muhc",
  "seq": 34,
  "of": 69,
  "text_len": 252,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgEM2ibr83AZ8hi6vi7u4woIKY4gmrCPurfn0EJoi5QKBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIVpZaT8="
}
---

adversarial review of day-rfc-foundations: BLOCK — RFC 0 publication and lifecycle checks plus RFC 1 conformance checks still accept states the corrected Plan requires them to reject, and the published correction leaves a contradictory live decision.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigi2ozmc2ludamsz26i5n4otoobfphztwsdp3uibq44wcvrdszomm",
  "sig": "e717a18e2223bba0a041761c0033bfcd64343168838e0a968a5788927ace17c42d307472e6d5bb578138420ca121a4c77d547bf7ddce184201dc58fe60c7efd0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze"
  ],
  "rev": "223mt67idzxxq",
  "seq": 35,
  "of": 69,
  "text_len": 441,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFCHthIo/Spo8GPNCrDYIlaEzmmrDbiZMVRosfFbI3clmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOha0xpbmVSYW5nZUF0g3ghc2NyaXB0cy9jaGVjay1yZmMwLXB1YmxpY2F0aW9uLnB5eChhOTdlNGQ0YzNlMDQ2NTAwYjVjNDUxMjJlZDgxMzFiN2EyNTZkODVjomNlbmQYOGVzdGFydBgkaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSFcn/dI"
}
---

BLOCK-F1: AC-3 publication evidence is not a claim-resolution fixture. scripts/check-rfc0-publication.py lines 36-48 copy the worktree file into a temporary directory and invent unsigned subject/path/sha256 JSON with no repository, commit, or CID; lines 50-56 query the current checkout rather than a fresh clone. Measured at a97e4d4: no rfc/0 claim exists under .claims, yet the checker exits 0, so claim/CID provenance cannot be falsified.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidbsdrrn4ha2ky7lszbrk6kz6yiduxiyplkay6j7ox2hosobsjoae",
  "sig": "7f9a5f5b90ed66d85e6dbf9f2dc8bca39c9e0698d228e3608eaa04e9a1912269605837f711c81e3aa95568ae22d9a128e1f1d2f575c44f2112292cdb8606632c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze"
  ],
  "rev": "223mt67iejxc5",
  "seq": 36,
  "of": 69,
  "text_len": 321,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFCHthIo/Spo8GPNCrDYIlaEzmmrDbiZMVRosfFbI3clmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOhZkZpbGVBdIJ4GnNjcmlwdHMvY2hlY2stcmZjcy1hZHJzLnNoeChhOTdlNGQ0YzNlMDQ2NTAwYjVjNDUxMjJlZDgxMzFiN2EyNTZkODVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSFcp/SZ"
}
---

BLOCK-F2: REQ-15 does not enforce an append-only number registry. scripts/check-rfcs-adrs.sh lines 78-82 compare only the current files with the current registry. In scratch, coherently renaming RFC 1 to RFC 2 and updating heading, index, and numbers.tsv exited 0 with the claimed registry guarantee still reported valid.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifmdbalzl3h5jfdrwjowat33bb3kb3irubgl6mbnxujbkhse2lkjq",
  "sig": "d0715c09f8dc7a62acfca6af7328a76ad3d7871089b2abe26a2667bc9030cb0b3ad42ac3bf3209c09a160cbfaca673e5a2c204a9201c8f0f94ac07e41a239e7f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze"
  ],
  "rev": "223mt67ieznco",
  "seq": 37,
  "of": 69,
  "text_len": 354,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFCHthIo/Spo8GPNCrDYIlaEzmmrDbiZMVRosfFbI3clmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOha0xpbmVSYW5nZUF0g3gac2NyaXB0cy9jaGVjay1yZmNzLWFkcnMuc2h4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOiY2VuZBgqZXN0YXJ0GCRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIVyvzKs="
}
---

BLOCK-F3: Accepted-state review metadata is presence-only. scripts/check-rfcs-adrs.sh lines 36-42 reject only the exact placeholders Not opened and Not scheduled and never validate dates or overrides. In scratch, Accepted plus Discussion x, Review-period-ends not-a-date, and Review-override forged exited 0, bypassing RFC 0 lines 90-96 and REQ-15/AC-15.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid6lcfjefjxowjvbii6g7krbuzroj4dsfn6tvaza653j4qxdlwm3u",
  "sig": "d1e842509bdd2f74091f2f3cea455e95b2856b50f7536cdfb0c43f86374f333059052489e188aeef22349ca9be56516f56c3cc135be3897c924d6197bdf0110e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze"
  ],
  "rev": "223mt67ifjaut",
  "seq": 38,
  "of": 69,
  "text_len": 412,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFCHthIo/Spo8GPNCrDYIlaEzmmrDbiZMVRosfFbI3clmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOhZkZpbGVBdIJ4HXNjcmlwdHMvY2hlY2stcmZjMS12ZWN0b3JzLnB5eChhOTdlNGQ0YzNlMDQ2NTAwYjVjNDUxMjJlZDgxMzFiN2EyNTZkODVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSFct5rw"
}
---

BLOCK-F4: The RFC 1 checker does not type-check its declared bridge or enforce per-component correspondence. scripts/check-rfc1-vectors.py lines 49-54 compare hard-coded composite strings without reading the bridge array, and lines 25-33 certify one coordinate for two material components. Scratch mutations A2:WRONG->X2 and deletion of one of two coordinates both exited 0, violating REQ-16 and AC-6/AC-7/AC-16.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicz732ikosba7ahzve6szy4bu6avxyciypyaaohxqiyv75lmoyoay",
  "sig": "18bed50893d03e244be5b54cf1afe9e3e98edc5de83bb12718cc2afb56fc624f3c9972b485eae5cb3375c8fb185ce163226ba62f2d3b1cc1991c5d434f744ad5",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze"
  ],
  "rev": "223mt67ifyzce",
  "seq": 39,
  "of": 69,
  "text_len": 547,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFCHthIo/Spo8GPNCrDYIlaEzmmrDbiZMVRosfFbI3clmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOha0xpbmVSYW5nZUF0g3glcmZjcy8xLWZyYW1lLWluZGV4ZWQtcHJvY2Vzcy1tb2RlbC5tZHgoYTk3ZTRkNGMzZTA0NjUwMGI1YzQ1MTIyZWQ4MTMxYjdhMjU2ZDg1Y6JjZW5kGQI7ZXN0YXJ0GQIQaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSFcv3yb"
}
---

BLOCK-F5: Required RFC 1 acceptance evidence remains absent. Design AC-6 requires identity, present, local-cell, pasted-cell, and typeable-without-realization vectors, but rfcs/1-frame-indexed-process-model.md lines 548-550 explicitly leave local/no-cell examples as non-executable Review obligations. AC-9 requires each migration vector to expose transported and lost data, while rfcs/vectors/1-process-model.json lines 22-27 carry only coarse flags. AC-10 requires laws and discriminating examples, while RFC 1 lines 552-571 only list questions.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihrwuho5sxh3xswoyi2ts5wzizysnhmlod3oimu3b3n6qgyjv62ri",
  "sig": "3d213a2f85e5c38645d250999359e9e118f257dade2e55e970b9279afafcf45e6d3c33aa0e59a46f006d57a1df14636e95990ea9e3bec7bff9bf38860bb3f0d0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiauehwyjcr7jkndyghtikwdmcevuezzu2wdnyteyvi2fr6fnsg5ze"
  ],
  "rev": "223mt67igisiv",
  "seq": 40,
  "of": 69,
  "text_len": 458,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFCHthIo/Spo8GPNCrDYIlaEzmmrDbiZMVRosfFbI3clmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE5N2U0ZDRjM2UwNDY1MDBiNWM0NTEyMmVkODEzMWI3YTI1NmQ4NWOhZkZpbGVBdIJ4UC5jbGFpbXMvZGF5LXJmYy1mb3VuZGF0aW9ucy96RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duLm1keChhOTdlNGQ0YzNlMDQ2NTAwYjVjNDUxMjJlZDgxMzFiN2EyNTZkODVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSFcx2Fy"
}
---

BLOCK-F6: The published correction leaves contradictory live decisions without direct supersession. The day-rfc-foundations projection line 268 says legacy witness lists default to sufficient; line 664 says they remain flat component reports. RQ-10 cites only the new Plan, not the old RQ-9 Decision, contrary to CLAUDE.md lines 470-477 requiring supersession on the subject a reader reaches. The record alone does not say which live Decision was superseded.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihp46iwsueqp4gez57tk7ovl5d52g2iuc67ab6rcq6tkki7hikxfe",
  "sig": "48f14c231995dd12550d2eec075135a0b37e2c00582fcbe0419dfb9f12acf29d65c308256c87142b7768286557c20bc1c20b4d1d461a6f4ea068b95c7aed9f65",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt67immd2z",
  "seq": 41,
  "of": 69,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChhOTdlNGQ0YzNlMDQ2NTAwYjVjNDUxMjJlZDgxMzFiN2EyNTZkODVjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSFdKSO3"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaqwtporjy3drtghrax5dkbiqry4jtz23od2wyvkosaiuhv3zkun4",
  "sig": "600ff919dfd512f1ec1155a17e6b379c8e60758d8c22ad8bc2c0e8d2c2ce79685e44f904789b3cc1924b6a148010878f90ce6832b24dc535cbdf18167dda5b0e",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaafsq7dnugfdc5qyldwlrylug6ms2moz7rnolsrj5otj5gdhpxqq",
    "bafyreihrwbewfmcyjyg6ora6stostfdd4nzlawfqxuplx7ofbhwijlfghe"
  ],
  "rev": "223mt6axs4g5g",
  "seq": 42,
  "of": 69,
  "text_len": 173,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOC2CpYJQABcRIgACyh8baGKMXYYWOy44XQ3mS0x2fxa5cop66aemGd94TYKlglAAFxEiDxsElisFhODedEHpTdKZRj43KwWLC9Hrv9xQnshKymOWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHNkYXktcmZjLWZvdW5kYXRpb25zaWFydGlmYWN0c4GhZkNvbW1pdHgoYmI3MWU3MGE5YjQxNTA0ZDVmNWY4YTEwODY4MjI4N2I0OTQ2N2EzY2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkhu4Ev+g=="
}
---

RQ-14: RQ-10 supersedes the earlier RQ-9 decision. Legacy witness lists are flat component reports and are not sufficient systems without an explicit versioned relationship.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig53fruw3xqyvl43ow5irmbggts2po635vlmplmpqr42dzosjlt2i",
  "sig": "c4d95c512d1c109b6054b723d25c13f842747950e4e4f131bd16811146561bc04cb1d89973d0d461ebbebc3db624a679e5c9d86a2e64d79c7e1a8521a4adf0c7",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiayetzvurbljt7f32vlgkr4ako4my2cyta4uuus2pkebr73wdi3va"
  ],
  "rev": "223mt6axu6imx",
  "seq": 43,
  "of": 69,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgGCTzWkQrTP5d6qsyo8Ap3GY0LEwcpSktPUQMf7sNG6hmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGJiNzFlNzBhOWI0MTUwNGQ1ZjVmOGExMDg2ODIyODdiNDk0NjdhM2Npd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIbuiOek="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 19068:0f3aeff7f40b448e]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiczmk6hge6qlnkranykglp3qeadqlnlgkixtwrvephatbf5zvbzwa",
  "sig": "dd5ab69874008bc9bf14bad585c659ece10204cc206cbec30d854f50e0ac4bb64d30a246390ae40d68130f7bdec719cc7fd856136a85fc8a96b205e7726e4b1c",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreig53fruw3xqyvl43ow5irmbggts2po635vlmplmpqr42dzosjlt2i",
    "bafyreiaqzwrg5pzxagpsdc5l4lxo4mfaqkmoecnlbd52w7t5aqtirokaua"
  ],
  "rev": "223mt6axupcxz",
  "seq": 44,
  "of": 69,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiDd2WNLbvDFV8263URYExpy093t9qtj1sfCPNDy6SVz0tgqWCUAAXESIBDNom6/NwGfIYur4u7uMKCCmOIJqwj7q359BCaIuUCgZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChiYjcxZTcwYTliNDE1MDRkNWY1ZjhhMTA4NjgyMjg3YjQ5NDY3YTNjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSG7qqNK"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiczzsljjy746qt4mupwfkxbrsxcxrulmaabwn4kg7l5vmjs4trd5y",
  "sig": "ddbd5af5fd78b9dda3800f7a1ad630297bcef132dfa086ec0d4d13e469e936e61a681863bde60c10787260af605d1fda2b59a87a7e73d478292a68b49d65e691",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt6axv2wyz",
  "seq": 45,
  "of": 69,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDJsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChiYjcxZTcwYTliNDE1MDRkNWY1ZjhhMTA4NjgyMjg3YjQ5NDY3YTNjaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSG7sHNx"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicklk2nocw26me5gjrd2ol3pjp4wj7krjyqbtatit5o4vtvadkarq",
  "sig": "44b5853b27b44b34655779466c5b824a9ffccaca64d7a61ddf491ccd17842d63114e7eff4ddc9aad6998ba377292d387d3bc2a53a2135d1edc7af942e5f5de90",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt6axvx46q",
  "seq": 46,
  "of": 69,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGJiNzFlNzBhOWI0MTUwNGQ1ZjVmOGExMDg2ODIyODdiNDk0NjdhM2Npd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZIbu+iCM="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigdrn2735ctc4yarv4fdwrcoqeru7xoonw732k2mwttb4qbtlhoze",
  "sig": "1037fbdbb439e91bbfa196a4af000090315d005125169996f8e4225ddb3f8afe6eb42e5bd9ba47937ec684e463970c11b0573a1fb9b25459e41b60bed0afe7e8",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiczmk6hge6qlnkranykglp3qeadqlnlgkixtwrvephatbf5zvbzwa"
  ],
  "rev": "223mt7fttpglj",
  "seq": 47,
  "of": 69,
  "text_len": 294,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgWWK8cxPQW1UQNwoy37gQA4LasykXnaNSPOCYS9zUObBmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKvOascI="
}
---

adversarial review of day-rfc-foundations: BLOCK — Round-2 RFC foundations remain unshippable: fresh-clone publication depends on a gitignored private identity, historical allocation and review-lifecycle mutations still survive, and RFC 1 vectors still accept ill-typed or invented semantics.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidr7g7j75itw7qb4qmmzc3j2kkhkh77r77bomtsy7p2uley2c6ozm",
  "sig": "f6c2fd284876d67aa8f57e2d3ee5722e68d112e5f0ebce7e81a50b90fcff48280788a7b235c6ae7c31297faca9f88d9e8592ad0c6c415a0fd3280d9b7203c646",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigdrn2735ctc4yarv4fdwrcoqeru7xoonw732k2mwttb4qbtlhoze"
  ],
  "rev": "223mt7fus6lzo",
  "seq": 48,
  "of": 69,
  "text_len": 694,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgw4t1/fRTFzAI14UdoidAkafu5zbf3pWmWnMPIBms7slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOha0xpbmVSYW5nZUF0g3ghc2NyaXB0cy9jaGVjay1yZmMwLXB1YmxpY2F0aW9uLnB5eCg2ZjhiY2I5YTEwNjM5MWNkMGJhZDc0YzQ4ODY1YzAwOTBiZWU1NDUzomNlbmQYOGVzdGFydBdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKvWCR4o="
}
---

BLOCK-R2-F1: AC-3 publication evidence is non-hermetic and does not prove publication or repository provenance. scripts/check-rfc0-publication.py lines 49-53 requires root/.kan/identity, which is gitignored and absent from a no-local clone; running scripts/check-rfcs-adrs.sh in that clone exits 1 with "the fixture author's signing identity is unavailable", so CI lines 108-111 cannot validate it. In-memory hostile removal of every Publication claim and of the Workspace artifact both still exits 0 because validate() checks only Decision, FileAt, digest, and a Closed status. Exact CID, commit, path, and digest mutations are caught, but the required published exact-repository claim is not.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifjubmc54mtdfzldyv2ixu3vmxynwmtizvnkbuzzzfg72ackucrvq",
  "sig": "12600ec62a53ace74388fae5b57722d0a2fc13ae35c1239b674073b3707da0884925566f4a500a3bc33d259daf354582e3e3edaf0fef262b3ead039de05deb22",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigdrn2735ctc4yarv4fdwrcoqeru7xoonw732k2mwttb4qbtlhoze"
  ],
  "rev": "223mt7fuspjz3",
  "seq": 49,
  "of": 69,
  "text_len": 573,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgw4t1/fRTFzAI14UdoidAkafu5zbf3pWmWnMPIBms7slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOha0xpbmVSYW5nZUF0g3gac2NyaXB0cy9jaGVjay1yZmNzLWFkcnMuc2h4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOiY2VuZBhmZXN0YXJ0GFtpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKvWKv3c="
}
---

BLOCK-R2-F2: REQ-15 still does not enforce an append-only RFC allocation in production. scripts/check-rfcs-adrs.sh lines 91-102 reads a baseline only from DAY_RFC_BASE_REGISTRY or main:rfcs/numbers.tsv; main predates this registry, while the self-test alone injects a copied baseline at lines 125-140. In a no-local scratch clone, coherently renaming RFC 1 to RFC 2 in the filename, heading, index, and numbers.tsv makes DAY_RFC_PUBLICATION_SKIP=1 scripts/check-rfcs-adrs.sh exit 0. The exact prior historical-renumber attack therefore still survives outside the self-test.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicrkkhu3kxpxcsfldcn62rfyvwqlolvxbvrb5lcloxf4d2ir2poii",
  "sig": "60056c1b613169975aa1f9cef33a575f68906430cd7226f02e7b52cadebf1eca4b20c13bd9ec2b2c2922530e562eaa7d1ad168142857bbb96f178a3d444ba98d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigdrn2735ctc4yarv4fdwrcoqeru7xoonw732k2mwttb4qbtlhoze"
  ],
  "rev": "223mt7futajwp",
  "seq": 50,
  "of": 69,
  "text_len": 678,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgw4t1/fRTFzAI14UdoidAkafu5zbf3pWmWnMPIBms7slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOha0xpbmVSYW5nZUF0g3gac2NyaXB0cy9jaGVjay1yZmNzLWFkcnMuc2h4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOiY2VuZBgxZXN0YXJ0GCRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKvWTPyA="
}
---

BLOCK-R2-F3: RFC 0 review acceptance remains forgeable despite stricter syntax. RFC 0 lines 90-96 requires 72 continuous hours on a proposal pull request, restart after substantive changes, and a unanimous time-only override; scripts/check-rfcs-adrs.sh lines 36-49 proves only any whitespace-free https URL, an arbitrary past timestamp, and a formatted override or None. A scratch RFC 0 marked Accepted with Discussion https://example.com/not-a-pull-request, Review-period-ends 2000-01-01T00:00:00Z, Review-override None, and a matching index exits 0. The combined self-test fails first on malformed Discussion and cannot demonstrate the date or override branches independently.
***8<***
---
{
  "v": 3,
  "cid": "bafyreih3zrcjx57tl6wukklr3esldu7wje432bxyngiuxetwi6vzacmrzi",
  "sig": "d42d8060d0ecf3018bec6a46abfa523c8fce8bb96a17d369c0f1f1d967767fc6661a91e3aa18c0fab88eaea9d898d78ff36dcb8c6a8a68d21a57ed75e89c67ca",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigdrn2735ctc4yarv4fdwrcoqeru7xoonw732k2mwttb4qbtlhoze"
  ],
  "rev": "223mt7futrmdg",
  "seq": 51,
  "of": 69,
  "text_len": 742,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgw4t1/fRTFzAI14UdoidAkafu5zbf3pWmWnMPIBms7slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOha0xpbmVSYW5nZUF0g3gdc2NyaXB0cy9jaGVjay1yZmMxLXZlY3RvcnMucHl4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOiY2VuZBhOZXN0YXJ0GDFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKvWbyME="
}
---

BLOCK-R2-F4: RFC 1 realization vectors remain not type-checked end to end. scripts/check-rfc1-vectors.py lines 52-75 derives atom boundaries but compares expected_realization to the hard-coded text A2 composed with A1 and never relates the expression back to atom names, present/target predicate names, endpoint cells, or each cell's exists flag. Renaming A1 to BROKEN consistently in atoms, bridge_order, and local-cell references while leaving the realization expression stale exits 0; separately, changing the first source predicate and final target predicate to WRONG and setting the first cell exists=false also exits 0. The exact prior A2 boundary mutation is now caught, but AC-6 and REQ-16 composition typing remain falsely certified.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifryyc3d7n5rmpgz4wus3aj72hz2ouimo6hjcf2zkdgchim6wdnqa",
  "sig": "ead27f9a564a5e5520e3089cb7f02b485455b37ba5622c0dedcc7bba355981be439f915f78173a336c73aa4703654acf2bf23ba082ab3ce93119c9cfedf4951a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigdrn2735ctc4yarv4fdwrcoqeru7xoonw732k2mwttb4qbtlhoze"
  ],
  "rev": "223mt7fuucnr3",
  "seq": 52,
  "of": 69,
  "text_len": 651,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgw4t1/fRTFzAI14UdoidAkafu5zbf3pWmWnMPIBms7slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOha0xpbmVSYW5nZUF0g3gdc2NyaXB0cy9jaGVjay1yZmMxLXZlY3RvcnMucHl4KDZmOGJjYjlhMTA2MzkxY2QwYmFkNzRjNDg4NjVjMDA5MGJlZTU0NTOiY2VuZBguZXN0YXJ0E2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkq9aRObQ=="
}
---

BLOCK-R2-F5: RFC 1 witness and migration vectors accept values outside the normative finite vocabularies. witness_outcome at lines 19-36 maps every unknown component outcome to not-certified, and migration_outcome at lines 39-46 maps arbitrary comparison, procedure, and coordinate strings to lossy. Mutating missing-sufficient outcome to invented and unsupported-procedure comparison, procedure, and coordinates to invented still makes the checker exit 0. Thus the checker can certify a machine-readable vector it did not semantically parse, violating REQ-16 and the honest-reads telos even though the named mismatch/equivalence mutations are caught.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibx4g6rqisy2m5t6clntzkh7vf6hztqpgo3fabnue7i4ms6opeygi",
  "sig": "7d65753b67eb4ec316c71e1b937584927c9508506ee593516b07d0f25aaffb2075f912ef0ec1ccc1c4b018e12f2b36d0e9f46576fe924f3e67a1f577fa2bd73f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt7fuxvfzq",
  "seq": 53,
  "of": 69,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCg2ZjhiY2I5YTEwNjM5MWNkMGJhZDc0YzQ4ODY1YzAwOTBiZWU1NDUzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSr13a+N"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiesfnp37osudv3nzyr3rmr3cvnjcxh64ndotp376xu7usygk2pndm",
  "sig": "6ecd14faa51c896cb2852cbefeb38cc1c96ba16bc9f08c1f147a68a929879c1867e321920774736eb3a801878b6be06fd0a454a1faadc22e4897809da5e2b5a4",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreig53fruw3xqyvl43ow5irmbggts2po635vlmplmpqr42dzosjlt2i"
  ],
  "rev": "223mt7gbvnzh2",
  "seq": 54,
  "of": 69,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg3dljS27wxVfNut1EWBMactPd7farY9bHwjzQ8uklc9JmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGYyMWExNzBhYTliNjRjNGZmNzdmMmU1OTRjOTQzY2RhYzY3MjQ1ODJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKw+5+fw="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 19331:53d457b5a9af23e0]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaexsfqh34krv4bwgnp6nf76zjgrcrb7tuhbtljw7dnlp5hghwze4",
  "sig": "48c04a189492e78fd940822c266d3995b83111e33be2f39b99fd95ec7e6b14c60fe2931b6ffcc2b35f0619ef7f88620ecaff3dd3e501a68fe9f646b01a08121b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreiesfnp37osudv3nzyr3rmr3cvnjcxh64ndotp376xu7usygk2pndm",
    "bafyreiczmk6hge6qlnkranykglp3qeadqlnlgkixtwrvephatbf5zvbzwa"
  ],
  "rev": "223mt7gbw7akc",
  "seq": 55,
  "of": 69,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiCSK1+/ulQddtziO4sjsVWpFc/uNG6b9/9en6SwZWntG9gqWCUAAXESIFlivHMT0FtVEDcKMt+4EAOC2rMpF52jUjzgmEvc1DmwZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChmMjFhMTcwYWE5YjY0YzRmZjc3ZjJlNTk0Yzk0M2NkYWM2NzI0NTgyaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSsPwpmf"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreif7gs2unvlztnst27vxemqhwoue2bnz25bmapzdfco74jrf2tffzq",
  "sig": "91480463f450f0266dd6ebec94501a02c2d50b0b887fbc086837921f283463790c8f84d0e388a73d117c6fe54e748d657e9ba66d036c0f280144238a6cb89d23",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt7gbwksl3",
  "seq": 56,
  "of": 69,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDNsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChmMjFhMTcwYWE5YjY0YzRmZjc3ZjJlNTk0Yzk0M2NkYWM2NzI0NTgyaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSsPyGG5"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihd2xlglr7i4n2xibq3c4cje5tbu2aozi7oak6ftvut35h6l6e6gq",
  "sig": "bbd123f5e51028e2b25a18e7829dab9a9a7d8128155c31dc3a7cbf58da73c2fa087dee5d969ed2adceb7770f95bdaaf522fe80e8ad6de3311c216c6b8745754d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaexsfqh34krv4bwgnp6nf76zjgrcrb7tuhbtljw7dnlp5hghwze4"
  ],
  "rev": "223mt7gbxhhv5",
  "seq": 57,
  "of": 69,
  "text_len": 254,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBLyLA++KjXgbGa/zS/9lJoiiH86HDNabfG1b+nMe2SdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGYyMWExNzBhYTliNjRjNGZmNzdmMmU1OTRjOTQzY2RhYzY3MjQ1ODJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKw/WtvQ="
}
---

RQ-15: Accepted-state validation is evidence-bearing rather than presence-only: review timestamps must demonstrate 72 elapsed hours (or name a structured unanimous override), and the exact day GitHub proposal PR must be readable and contain the RFC file.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiecqqd6isblzdvqdrtmpjhz3albchmw5zwsmh64l5gtqzvpxxejay",
  "sig": "d77e4c5b13dd46dbc088263b633fd3f8574539bb8e58895dbc24d6959ff3790f338161454b472cd74b5c91d33056b470de30952092cba3a22841c8617071d563",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7gbxyxw2",
  "seq": 58,
  "of": 69,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGYyMWExNzBhYTliNjRjNGZmNzdmMmU1OTRjOTQzY2RhYzY3MjQ1ODJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZKw/fdws="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigbcavzihxi7wchwjqyg7jfs5gupbr5ehwflpwvdgliobguxgbqfm",
  "sig": "733c0b84c943dd0a76ed3f62e9a4f71a41aebc7bd704a3d830f236e3a4b08fbc257b3cf4884bc93efb380751afd1c561906c098350d324789d0f67c9be44d0b6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreiaexsfqh34krv4bwgnp6nf76zjgrcrb7tuhbtljw7dnlp5hghwze4"
  ],
  "rev": "223mt7iczyhdy",
  "seq": 59,
  "of": 69,
  "text_len": 210,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgBLyLA++KjXgbGa/zS/9lJoiiH86HDNabfG1b+nMe2SdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGE2N2ZjNzVjOTBiYWI4N2FiNmY5NmJjMjk4MDkzYWFlZWY4MWY3YzVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK5H/NMY="
}
---

adversarial review of day-rfc-foundations: BLOCK — Round 3 remains unshippable: PR chronology is forgeable through commit dates/order, and RFC 1 vectors still certify unparsed witness and migration semantics.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic2v2wdcqe3dwtesd4m5jdworun4fcfshj2trd6wscp3eojdwntsm",
  "sig": "cbc3e66ca3b9398e6e9b1df1fd9edc7c0513d86921b186ee13c21d0a97d826bd409ab423e46d247b54ffff5fcc096a402ae1e54d4db48bc7a37373e009468055",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigbcavzihxi7wchwjqyg7jfs5gupbr5ehwflpwvdgliobguxgbqfm"
  ],
  "rev": "223mt7idp4tzq",
  "seq": 60,
  "of": 69,
  "text_len": 713,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwRArlB7o/YR7Jhg30ll01Hhj0h7FW+1RmWhwTUuYMCtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE2N2ZjNzVjOTBiYWI4N2FiNmY5NmJjMjk4MDkzYWFlZWY4MWY3YzWha0xpbmVSYW5nZUF0g3gbc2NyaXB0cy9jaGVjay1yZmMtcmV2aWV3LnB5eChhNjdmYzc1YzkwYmFiODdhYjZmOTZiYzI5ODA5M2FhZWVmODFmN2M1omNlbmQYTmVzdGFydBgiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSuTUWd2"
}
---

BLOCK-R3-F1: Accepted-state chronology is still forgeable. scripts/check-rfc-review.py lines 34-56 asks GitHub only for files and commits, then treats max(committedDate) as the latest commit; it never checks PR createdAt or the commit-list HEAD. Executed coherent PR responses with an appended HEAD whose committer date was backdated before an earlier commit: both 72-hour metadata with no override and a one-hour unanimous override naming the older-by-order OID exited 0. Thus an old commit in a newly opened PR can claim review before the PR existed, and an appended backdated substantive HEAD neither restarts the clock nor becomes the required override OID, violating RFC 0 lines 91-102 and REQ-15/AC-2/AC-15.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigdn7gx2c3rw4fyxerrlepafr64sjtgiajs6oqqdw6g2g7d4iwjwq",
  "sig": "27a4613abccd9d9e071c6b4d519e3e2bd704d06caf164abf4cf44aab9b78e2a77e37711edf2d97caef23c9efefd9047233188da53db0610593400c58a691b827",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigbcavzihxi7wchwjqyg7jfs5gupbr5ehwflpwvdgliobguxgbqfm"
  ],
  "rev": "223mt7ids7lia",
  "seq": 61,
  "of": 69,
  "text_len": 644,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwRArlB7o/YR7Jhg30ll01Hhj0h7FW+1RmWhwTUuYMCtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE2N2ZjNzVjOTBiYWI4N2FiNmY5NmJjMjk4MDkzYWFlZWY4MWY3YzWha0xpbmVSYW5nZUF0g3gdc2NyaXB0cy9jaGVjay1yZmMxLXZlY3RvcnMucHl4KGE2N2ZjNzVjOTBiYWI4N2FiNmY5NmJjMjk4MDkzYWFlZWY4MWY3YzWiY2VuZBhlZXN0YXJ0E2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkrk4K16w=="
}
---

BLOCK-R3-F2: RFC 1 witness vectors still accept structures the checker does not parse. scripts/check-rfc1-vectors.py lines 19-25 returns for legacy, necessary, and exact relationships before validating components or finite outcomes; lines 90-101 collapse case IDs through a dictionary and infer evidence count from set(value) without validating that value is an evidence-CID list or that the artifact exists. Hostile mutations replacing a legacy outcome with invented, replacing evidence_cids with the string abcde, deleting the artifact, and duplicating a vector ID each exited 0, contradicting RFC 1 lines 306-331 and 538-562 and REQ-16/AC-7.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicvwu343isvzl3bojinbphabf2iqopk4bdk753t23n72cppo4yer4",
  "sig": "242ea436d93bb0d36b59ad1f8173d1db17b517b124c4bd7d7512daaf269389296c6047e21c2def7c0cbf27e2bb89788fa51ed706422701d291f2850fea8dfee5",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigbcavzihxi7wchwjqyg7jfs5gupbr5ehwflpwvdgliobguxgbqfm"
  ],
  "rev": "223mt7idtgrdz",
  "seq": 62,
  "of": 69,
  "text_len": 585,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwRArlB7o/YR7Jhg30ll01Hhj0h7FW+1RmWhwTUuYMCtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KGE2N2ZjNzVjOTBiYWI4N2FiNmY5NmJjMjk4MDkzYWFlZWY4MWY3YzWha0xpbmVSYW5nZUF0g3gdc2NyaXB0cy9jaGVjay1yZmMxLXZlY3RvcnMucHl4KGE2N2ZjNzVjOTBiYWI4N2FiNmY5NmJjMjk4MDkzYWFlZWY4MWY3YzWiY2VuZBh1ZXN0YXJ0GGdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK5OWXMk="
}
---

BLOCK-R3-F3: RFC 1 migration vectors certify invented output semantics. scripts/check-rfc1-vectors.py lines 103-117 validates only that transported and lost are lists and that lost is empty/nonempty by expected class; it never validates the named transported/lost objects against the case inputs or RFC vocabulary. Replacing the unsupported-procedure payload with invented-transport/invented-loss and making invertible-reindexing transport nothing both exited 0, while RFC 1 lines 554-562 says every vector exposes what actually transported and what was lost; REQ-16/AC-9 remain unmet.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicosmbq43iaetiyksprkaua6eyf6abpkewehq5tmdzfm5gftzsefi",
  "sig": "c72ba1a34c6a2f0d2785ca97ec59ea889fda4bb77a044e9ec316ea957e50fb483cf01b1d7e7ebc4fb60906b70095648a1f5152c88c7bf6ce4bf6e6359ba5f50d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt7idu4eyn",
  "seq": 63,
  "of": 69,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgqFmQ29tbWl0eChhNjdmYzc1YzkwYmFiODdhYjZmOTZiYzI5ODA5M2FhZWVmODFmN2M1oWtMaW5lUmFuZ2VBdIN4G3NjcmlwdHMvY2hlY2stcmZjLXJldmlldy5weXgoYTY3ZmM3NWM5MGJhYjg3YWI2Zjk2YmMyOTgwOTNhYWVlZjgxZjdjNaJjZW5kGE5lc3RhcnQYIml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkrk6ErXA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicyjvylm4gvh4levsizh45nuealx5a7inxlrxiuy4p54vhxul4b44",
  "sig": "670453c838b0051e7385ec637196bc4a6883d5393021aba058350c0b2e3cd54f5ccefdf943feb3ea1fd19f8b216c7b592f7fc6fc034b3e74d6f23ec19b4eeb1d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreiesfnp37osudv3nzyr3rmr3cvnjcxh64ndotp376xu7usygk2pndm"
  ],
  "rev": "223mt7iitq5qe",
  "seq": 64,
  "of": 69,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgkitfv7pUHXbc4juLI7FVqRXP7jRum/f/Xp+ksGVp7RtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGQ0MzY2MjRjMDNiZjY5ZWY4MzFjN2VhYjk1ODFiOTE4ZGU4YWUwMmZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK52bDmA="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 19604:3d8b5cc3792cbcc1]
***8<***
---
{
  "v": 3,
  "cid": "bafyreie3d6ex4t5qgvwkboxzpwi6t2axuec5bahvs5xzu3unffbi5wxw6u",
  "sig": "7ef5627bff1659eadc50957ad41039251c1d2d1a351838324b8dae72493571af3f8fdd2c96398d6db818b6ffa3e2bcfa74b19677569184fb24d0932b22af6be8",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreicyjvylm4gvh4levsizh45nuealx5a7inxlrxiuy4p54vhxul4b44",
    "bafyreiaexsfqh34krv4bwgnp6nf76zjgrcrb7tuhbtljw7dnlp5hghwze4"
  ],
  "rev": "223mt7iiuccif",
  "seq": 65,
  "of": 69,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiBYTXC2cNU/FkrJGT862hALv0H0NuuN0Uxx/eVPei+B59gqWCUAAXESIAS8iwPvio14Gxmv80v/ZSaIoh/OhwzWm3xtW/pzHtknZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNDM2NjI0YzAzYmY2OWVmODMxYzdlYWI5NTgxYjkxOGRlOGFlMDJmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSudpCFN"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreic4buhadukltpfulgqkqjimxmyamcptupx2etegx7rjyizvtexznm",
  "sig": "5584ed219c397ae9839ba6037389284210ae37b21329823f0d65e570e47b7391175e314f9d12cbe9669f4ac9ca7580dea6e6e3501c7cb57c9cd8d674a034cdf0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt7iiuobq6",
  "seq": 66,
  "of": 69,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDRsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNDM2NjI0YzAzYmY2OWVmODMxYzdlYWI5NTgxYjkxOGRlOGFlMDJmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSudqh5d"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiggcqg3xzyhjaja5owqo3ennqgakpq5x6jfyr2q6ptuyylgpjcqii",
  "sig": "ef6f073ff4467d510e7d7aa20931653a15b24f484196e5d648962ea74342b1df0d04b776f947d140faaeec1a780b36dd1c9f97e4f99fc74b82715e660f583253",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreie3d6ex4t5qgvwkboxzpwi6t2axuec5bahvs5xzu3unffbi5wxw6u"
  ],
  "rev": "223mt7iivniko",
  "seq": 67,
  "of": 69,
  "text_len": 264,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgmx+Jfk+wNWyguvl9kenoF6EF0ID1l2+abo0pQo7a9vVmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGQ0MzY2MjRjMDNiZjY5ZWY4MzFjN2VhYjk1ODFiOTE4ZGU4YWUwMmZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK525uaM="
}
---

RQ-16: Review chronology is anchored to GitHub's server-recorded push time for the PR head, not author-controlled commit dates. Every finite reference vector uses closed vocabularies and validates complete case structure before relationship-specific outcome logic.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiedx4fcpiqmtlrgelj6piskzbdob77u3qbaznmizvfdzcf6lhvxci",
  "sig": "6caba115f50b8c40b5c26eccc6d929d04db6203623766c67f858563860828e804f6f4addef62b3eb06f8756c57cbf0333d27a0eb3d91bb00b4da4a40493ef209",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7iiwbwch",
  "seq": 68,
  "of": 69,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGQ0MzY2MjRjMDNiZjY5ZWY4MzFjN2VhYjk1ODFiOTE4ZGU4YWUwMmZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK53D8As="
}
---
