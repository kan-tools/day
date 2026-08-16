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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
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
  "of": 16,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDJjZGEzYTliYTNlOWJkN2IxZjQxMTQ1MWIzZjU0NTkwZjZhYTAwYjFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZH7YA+Ds="
}
---
