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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
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
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGQ0MzY2MjRjMDNiZjY5ZWY4MzFjN2VhYjk1ODFiOTE4ZGU4YWUwMmZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK53D8As="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigcipc5ykfnjhngxrp2s5puobfjqy4iijt3t6afpa7poxskh53piy",
  "sig": "ced8913284ef0981a616913e3a52cf643f789ce51266b30f1aea49b5998a65c42cb9c9aecb281342b9dba7f7163698923d4ac3481433c1568acf4de15d4b23db",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreie3d6ex4t5qgvwkboxzpwi6t2axuec5bahvs5xzu3unffbi5wxw6u"
  ],
  "rev": "223mt7jb6dlgx",
  "seq": 69,
  "of": 106,
  "text_len": 261,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgmx+Jfk+wNWyguvl9kenoF6EF0ID1l2+abo0pQo7a9vVmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDk2NGUxMmMzOWY3NGRjNWM1NjI4ZDhhMzI2OTAxMmM5YmNjYzRiNzhpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK85ExS0="
}
---

adversarial review of day-rfc-foundations: BLOCK — Round 4 remains unshippable: GitHub returns null for normal PR-head pushedDate, evidence cases bypass complete witness validation, and migration vectors accept duplicate and semantically false transport/loss.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic3p6mhfxbihwo2wxxtozlomxdjofo6z76ru573wocppa22e4f43m",
  "sig": "f71cc7bdb8ea6281a2e6d3985763b0851c68e5dba203f37d361477f9c78c34a472db82a046b79769591d468451211d4d1f7f7b405064c93973214507a1d340b5",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigcipc5ykfnjhngxrp2s5puobfjqy4iijt3t6afpa7poxskh53piy"
  ],
  "rev": "223mt7jbmxxge",
  "seq": 70,
  "of": 106,
  "text_len": 547,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwkPF3CitSdprxfqXX0cEqYY4hCZ7n4BXg+915KP3b0ZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDk2NGUxMmMzOWY3NGRjNWM1NjI4ZDhhMzI2OTAxMmM5YmNjYzRiNziha0xpbmVSYW5nZUF0g3gbc2NyaXB0cy9jaGVjay1yZmMtcmV2aWV3LnB5eCg5NjRlMTJjMzlmNzRkYzVjNTYyOGQ4YTMyNjkwMTJjOWJjY2M0Yjc4omNlbmQYOWVzdGFydBgtaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSvPLvUV"
}
---

BLOCK-R4-F1: Accepted-state chronology is inoperable for normal GitHub PR heads. scripts/check-rfc-review.py lines 45-57 requires Commit.pushedDate and rejects null. Live GraphQL reads of recent day PR heads 218, 217, 216, 215, and 214 returned pushedDate:null for all five; executing the checker against PR 218 failed 'proposal head has no server-recorded push time'. The author-date backdating path is closed, but the replacement evidence is absent in the repository's actual workflow, so no ordinary Accepted transition can satisfy RQ-15/RQ-16.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigkb6lquvwys6skhixo4lnmtq2mpnro4ickbz7asmiwtaaxlgdfx4",
  "sig": "4e0d08cc2e806a4377047f150560c47397f20b27209b840ce30a3b90533ca9d17c4d906e8d9882bffc490a6ab5aa6ecaed0779edf2c0d3dbbcaba731f558164a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigcipc5ykfnjhngxrp2s5puobfjqy4iijt3t6afpa7poxskh53piy"
  ],
  "rev": "223mt7jbnjh4n",
  "seq": 71,
  "of": 106,
  "text_len": 625,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwkPF3CitSdprxfqXX0cEqYY4hCZ7n4BXg+915KP3b0ZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDk2NGUxMmMzOWY3NGRjNWM1NjI4ZDhhMzI2OTAxMmM5YmNjYzRiNziha0xpbmVSYW5nZUF0g3gdc2NyaXB0cy9jaGVjay1yZmMxLXZlY3RvcnMucHl4KDk2NGUxMmMzOWY3NGRjNWM1NjI4ZDhhMzI2OTAxMmM5YmNjYzRiNziiY2VuZBhoZXN0YXJ0E2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkrzzez3w=="
}
---

BLOCK-R4-F2: RFC 1 witness evidence remains structurally uncertified. scripts/check-rfc1-vectors.py lines 98-104 sends artifact-two-evidence and shared-evidence-reuse through only artifact/list/distinct-count checks and skips witness_outcome; lines 23-28 also accept a nameless singleton component. Executed mutations accepted an empty artifact evidence set, a one-entry shared-evidence case that no longer demonstrates reuse by two components, an invented expected result on the evidence case, and legacy-flat with its only component name removed. AC-7 and RQ-16 require these exact distinctions and complete case structure.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigvqqb536wdcoimze3iyleocjugrsldb4i7s7kn7qdbwqr6y2cv4y",
  "sig": "9a9d841b9c3a82e6f4a72b669021496b1cf53394eef288f6b9e3739a32e922dc7b24285655cb99032be93f3d4a292d672de7078af26f3c4a6af7e8fff8e3dc55",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreigcipc5ykfnjhngxrp2s5puobfjqy4iijt3t6afpa7poxskh53piy"
  ],
  "rev": "223mt7jbo2w4e",
  "seq": 72,
  "of": 106,
  "text_len": 685,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgwkPF3CitSdprxfqXX0cEqYY4hCZ7n4BXg+915KP3b0ZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDk2NGUxMmMzOWY3NGRjNWM1NjI4ZDhhMzI2OTAxMmM5YmNjYzRiNziha0xpbmVSYW5nZUF0g3gdc2NyaXB0cy9jaGVjay1yZmMxLXZlY3RvcnMucHl4KDk2NGUxMmMzOWY3NGRjNWM1NjI4ZDhhMzI2OTAxMmM5YmNjYzRiNziiY2VuZBh9ZXN0YXJ0GGppd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK89Ab88="
}
---

BLOCK-R4-F3: RFC 1 migration details are still asserted rather than derived. scripts/check-rfc1-vectors.py lines 106-125 builds a dict without rejecting duplicate IDs and constrains transported/lost only to a global vocabulary, disjointness, and empty/nonempty shape. Executed mutations accepted a duplicate migration ID, invertible transport reduced to only telos, an empty transported set for the supported invertible/unsupported-procedure case, and the semantically reversed claim that procedure transported while telos was lost. Unknown, empty-equivalent, and overlapping details now reject, but AC-9/AC-16 require case-specific migration outcomes and honest transported/lost data.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicgk2mbxzxmlo3kgy7gb64nk3avj4fm7mpnft7gaymu5zwidmvsqq",
  "sig": "196b270f4547869301a4792a703098d4edc5bed69a3c0cea3ca4021cb0844edb49e9dbf95b53915405a37fcd9e560229d9c2edab209bebb0789689a56c17f1e0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt7jbomq2l",
  "seq": 73,
  "of": 106,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCg5NjRlMTJjMzlmNzRkYzVjNTYyOGQ4YTMyNjkwMTJjOWJjY2M0Yjc4aXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSvPSVeg"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreienmnu7l6wnzusfknevv2bok2y3o3mmbs66bn7ceuuto7y4rnezle",
  "sig": "fd6d5634cf685b06e48301cb7688599d6f6aeb76c868e660005b018412eb5b763399fcf201e97f18441068ca22905e4c4520283bc77e39ee04f72bff6c02fa4f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreicyjvylm4gvh4levsizh45nuealx5a7inxlrxiuy4p54vhxul4b44"
  ],
  "rev": "223mt7jov65it",
  "seq": 74,
  "of": 106,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgWE1wtnDVPxZKyRk/OtoQC79B9DbrjdFMcf3lT3ovgedmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGVkMGI3NGZjMDlkNzViODZiNTAxNzFlZjhmZDViNDg3Y2QxZWVkZWZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK+myCkU="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 19931:48d90a358f814745]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihpsyckkmo7dpfqcxl4zyvmx4vzqh44wxhezji2geon6p5s4cbxn4",
  "sig": "ac0a58a3eb1a67ffc19791de0453525b092b5b14dd5078f28d02f9fcef3af62b39c6bd6a61ee2bb650a06ce9b3342f6c7f0fe911f761383d6c3988bc69dc3c9d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreienmnu7l6wnzusfknevv2bok2y3o3mmbs66bn7ceuuto7y4rnezle",
    "bafyreie3d6ex4t5qgvwkboxzpwi6t2axuec5bahvs5xzu3unffbi5wxw6u"
  ],
  "rev": "223mt7jovptuf",
  "seq": 75,
  "of": 106,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiCNY2n1+s3NJFU0la6C5WsbdtjAy94LfiJSk3fxyLSZWdgqWCUAAXESIJsfiX5PsDVsoLr5fZHp6BehBdCA9Zdvmm6NKUKO2vb1ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlZDBiNzRmYzA5ZDc1Yjg2YjUwMTcxZWY4ZmQ1YjQ4N2NkMWVlZGVmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSvpuubf"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreicbwf3fczokkepgrbyjyh3t6npx55nppfqmwzf55mrgyoq7pmf42u",
  "sig": "417a2eba52dd3ab09fa3820e511124a26b9536ecc66edf45a2bcb82411cf99b37409f823229958b8ac22b35aa7637e2584021ace07ba1accc70699384640b6d8",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt7jow3hyy",
  "seq": 76,
  "of": 106,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDVsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlZDBiNzRmYzA5ZDc1Yjg2YjUwMTcxZWY4ZmQ1YjQ4N2NkMWVlZGVmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSvpwLd6"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicvqx7a4afftel2bdp3h6o7254qgfjhqirgvbew5v7vm5nin5vv4i",
  "sig": "24e671366ee36cbb6e764f17cc3a3ebdc17dbc61444c32824c32cdd0428434446796eb82dfd0cdae0b81c9e41954053bd4b84246684d8f4c2a30c2930d4ef289",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreihpsyckkmo7dpfqcxl4zyvmx4vzqh44wxhezji2geon6p5s4cbxn4"
  ],
  "rev": "223mt7jox2wpt",
  "seq": 77,
  "of": 106,
  "text_len": 309,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg75YEpTHfG8sBXXzOKsvyuYH5y1zkylGjEc3z+y4IN29mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGVkMGI3NGZjMDlkNzViODZiNTAxNzFlZjhmZDViNDg3Y2QxZWVkZWZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK+nQck0="
}
---

RQ-17: Because GitHub's `Commit.pushedDate` is null for ordinary day PRs, review chronology uses the exact PR-head `committed` timeline event's server-issued signature-verification timestamp. Evidence and migration vectors additionally have exact closed case schemas and case-derived transport/loss semantics.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihcqkwgqm2narpbh4im3argyfljxb5oo7rzsqeixe4rnbpqhxwxcy",
  "sig": "df63981bca4a6a6cc639aab3eff46c5cc7261f065df81d00e6d23d7173bd24f7250e88da3e8df5c9265d14a549bcf40a7363f9fa1485a9174343f014f6ba3f1b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7joxmvfn",
  "seq": 78,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGVkMGI3NGZjMDlkNzViODZiNTAxNzFlZjhmZDViNDg3Y2QxZWVkZWZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZK+nZbO8="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreico6yzxim4jtyfut45eoz4vtp3tiboqxhmmsps23wqxsiwfgvwdvm",
  "sig": "6168b8ede26473360c4aa92fa96c4855cbd4705637d4b214a1017f7558051b1a4e6fd50a8863de9e41eb3c8af9ec5d198acc249d06fd6d7b8ded0e766cd75460",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreienmnu7l6wnzusfknevv2bok2y3o3mmbs66bn7ceuuto7y4rnezle"
  ],
  "rev": "223mt7kq67tsk",
  "seq": 79,
  "of": 106,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgjWNp9frNzSRVNJWuguVrG3bYwMveC34iUpN38ci0mVlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDkwY2RlYThhNjZlOTMyMzBhZGFjMTcwNWM5M2M2YmY3ZjY3YzRhN2Rpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLCxC5Dc="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 21315:9bfc08859b39d9c0]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihfxj6wqerhb327dhi4sgkxn7pdg3cffmvlhymi4z3kbe6mzcil7m",
  "sig": "6e7223dd07fa7cdd227b1b2fccab9ec0e70875a9b32d93a6887dd5ba2bbf0bfc71ee060e5b3f4ed6132ca4e1485208d1deb1a9a708fc25694d3edb20ee9249ad",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreico6yzxim4jtyfut45eoz4vtp3tiboqxhmmsps23wqxsiwfgvwdvm",
    "bafyreihpsyckkmo7dpfqcxl4zyvmx4vzqh44wxhezji2geon6p5s4cbxn4"
  ],
  "rev": "223mt7kq6sdrn",
  "seq": 80,
  "of": 106,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiBO9jN0M4meC0nzpHZ5Wb9zQF0LnYyT5a3aF5IsU1bDq9gqWCUAAXESIO+WBKUx3xvLAV18zirL8rmB+ctc5MpRoxHN8/suCDdvZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCg5MGNkZWE4YTY2ZTkzMjMwYWRhYzE3MDVjOTNjNmJmN2Y2N2M0YTdkaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSwsTCZ/"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiet4bjtcrzcbpofvmhwdx4ajoesn3uf2fmagaebnmbxflaohfabs4",
  "sig": "2b43491a65d65b43235ab24a416bc0333db797959792b153010a08585157581403c26433d9177976c84fb6d911d9b6c3a26c8691f286ec887e80c0c2eecd9c31",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt7kq76ghi",
  "seq": 81,
  "of": 106,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDZsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCg5MGNkZWE4YTY2ZTkzMjMwYWRhYzE3MDVjOTNjNmJmN2Y2N2M0YTdkaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWSwsUjFI"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreieg3xfhd6fjbubmgt45zm6ec4hq74rbxsrvai3l7uzpehzh62kjgu",
  "sig": "e0cc547f7539679d0a4f2bf71ea955f108f54c4dfde1aaa88d7fcee74411f18153e95c8cd848aabe7df1ddb94c8f815498aaa9b305560524ed0914db20acb8aa",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreihfxj6wqerhb327dhi4sgkxn7pdg3cffmvlhymi4z3kbe6mzcil7m"
  ],
  "rev": "223mt7kqa6r3h",
  "seq": 82,
  "of": 106,
  "text_len": 211,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg5bp9aBInDvXxnRyRlXb94zbEUrKrPhiOZ2oJPMyJC/tmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDkwY2RlYThhNjZlOTMyMzBhZGFjMTcwNWM5M2M2YmY3ZjY3YzRhN2Rpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLCxiW7g="
}
---

RQ-18: The indexed-process/equipment account receives a clean standalone rewrite under `rfcs/1/`, incorporated by RFC 1. Markdown-with-LaTeX is the claim-addressed source; HTML is a local derivative for reading.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie7gcivsx7qwdvgsq7ioaiw3ji45cdrpxksl6fjk6h7nemm23stei",
  "sig": "86b1287bdcf8b34e7614e11944991056ed5c53d6135fe1c0f7148ee4456d20fe26c4d44c9f9d1006f43b6e34e2702bcb091170d7abdd87af8dc27aa04715db9f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7kuskfuy",
  "seq": 83,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDFlMTU2ODg2MTViODIyY2I0YjBjODI0OGZmMTQ0OWMwM2Q0MzU2NTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLDWILv8="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiefxexfn4p3lbnhfrv2ryhiqcdvkrm7tyjjf62suuoaeu7ocfht6e",
  "sig": "9bc05b703d3dfbcb4b368aa4ea1c319dfd9497349a45bd7d4250b2a58e02861823eb6081112257f262cb73ef2ae5d81e4f573ff7dc643206c2070dd104f1c39d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreico6yzxim4jtyfut45eoz4vtp3tiboqxhmmsps23wqxsiwfgvwdvm"
  ],
  "rev": "223mt7o355ggq",
  "seq": 84,
  "of": 106,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgTvYzdDOJngtJ86R2eVm/c0BdC52Mk+Wt2heSLFNWw6tmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDI3NjkzNDQ5YmYzZGVlMTZlMTMzODIzYWRiMGIyZWFhMDcwNDg5NGZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLQIxsQ4="
}
---

design doc .design/day-rfc-foundations.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 23548:e77bc5f57b2eb166]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidkes5xyt4raykb4l7avc3henb5wiwus3i2alc3h5bfm5qx573guq",
  "sig": "350be2799eb3ec028f891c8e04b0e9f979ca2ff278311319abe83668ec1eac0e2f9ce64d3d678d5f63e86e5f5645b83b4f29aaad9554addc6d2b6a51c2475934",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "plan",
  "cites": [
    "bafyreiefxexfn4p3lbnhfrv2ryhiqcdvkrm7tyjjf62suuoaeu7ocfht6e",
    "bafyreihfxj6wqerhb327dhi4sgkxn7pdg3cffmvlhymi4z3kbe6mzcil7m"
  ],
  "rev": "223mt7o35ps4a",
  "seq": 85,
  "of": 106,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4LYKlglAAFxEiCFuS5W8ftYWnLGuo4OiAh1VFn54SkvtSpRwCU+4RTz8dgqWCUAAXESIOW6fWgSJw718Z0ckZV2/eM2xFKyqz4YjmdqCTzMiQv7ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCgyNzY5MzQ0OWJmM2RlZTE2ZTEzMzgyM2FkYjBiMmVhYTA3MDQ4OTRmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWS0COt/Y"
}
---

day-rfc-foundations design (.design/day-rfc-foundations.md): Establish a reviewed RFC and ADR discipline for day, then use its first substantive RFC to specify day's central process ontology before further v0.13 implementation. RFC 1 will distinguish an aspirational category-theoretic denotational model, a precise operational profile for current day, and an explicit approximation map between them; acceptance of that contract blocks v0.13 implementation, while implementation of the full formal target does not. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihlrtomm5tdfyx4d2pa6d6ghzed567b5tqdtmw2espewmrezpox6q",
  "sig": "519dccf976b15dbd8dcffa228fa0be8a049b7783bad64adc7adf42670c2103b43228fc1a6d9c2b6d73e3afc3261e30ac696d9e7066e0984b507da9d15371e850",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mt7o363ktf",
  "seq": 86,
  "of": 106,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhQRmVhdHVyZTogRGF5IFJGQyBmb3VuZGF0aW9ucyBhbmQgcHJvY2Vzcy1tb2RlbCBzcGVjaWZpY2F0aW9uLCBjb3JyZWN0aW9uIHJvdW5kIDZsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCgyNzY5MzQ0OWJmM2RlZTE2ZTEzMzgyM2FkYjBiMmVhYTA3MDQ4OTRmaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWS0CQMK8"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihg5t6sakoighgvbnlnibb66udf5y5p3so4ydm6o6t4i6splk5e4i",
  "sig": "adcc8195135d6a4e1a1dc79308d943c31c0f29cd1f101b0072aed5bc4a53887f11ce28dee0bbab458d0265897a224526524db2a09a6c6db0d1ffedce8343148b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreidkes5xyt4raykb4l7avc3henb5wiwus3i2alc3h5bfm5qx573guq"
  ],
  "rev": "223mt7o373sml",
  "seq": 87,
  "of": 106,
  "text_len": 393,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgaiS7fE+RBhQeL+CotnI0PbItSW0aAsWz9CVnYX7/ZqRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDI3NjkzNDQ5YmYzZGVlMTZlMTMzODIzYWRiMGIyZWFhMDcwNDg5NGZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLQJQ4eQ="
}
---

RQ-19: The companion is strengthened into a definition–proposition–proof sketch with concrete instances and boundary-explicit diagrams. A statement is called a proposition only relative to declared assumptions; unresolved equipment, enrichment, tensor, representability, or adjoint choices appear as open obligations and do not acquire normative force from mathematical presentation alone.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifqb6247n4crqvvaxdumne3xv3eydfqt5mxivf34yse7snis2jnry",
  "sig": "96ea6bdee9b728d4b8ed4c00005660383497b29628c3261bb42f58884e8080ae68bf4d8aaede2737f1a63f2bb76d4fd50a7c315f2bc4cb845ee087dc2b4be97a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7oj3nomj",
  "seq": 88,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDQzMDU5MWVjMmYxYjk5ZjlhNDY4MGQzNTI2OTEwYTllOWEyYmY1OWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLR4Z0eo="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibwt7cb7li23lkurpsknnzuxu7ligdloqotq5cp2oygeseaf6ak24",
  "sig": "6f4723da8ad843e0c1d671f169b7c0fc2d7e34afb3be74f7ca211eb84ac9f82b2796667ed222c5d4c0cc9f7b5a62953e15287c0118b60b94fdd991922bf3c886",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreidkes5xyt4raykb4l7avc3henb5wiwus3i2alc3h5bfm5qx573guq"
  ],
  "rev": "223mt7pudrawa",
  "seq": 89,
  "of": 106,
  "text_len": 251,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgaiS7fE+RBhQeL+CotnI0PbItSW0aAsWz9CVnYX7/ZqRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGJkZmY1ZTQ0ODk5MmQxNTE2YjYyYWNmMDE0MWIxYmY0MmM1NGYzYzJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLXSbmNU="
}
---

adversarial review of day-rfc-foundations: BLOCK — RFC 1 companion acceptance is not enforced: stale HTML and a missing exact-address publication projection both pass the shipped validator; the workspace test suite also has four unaccounted commits.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibplybwzfhjsjozexsiwjs44wik3wh265hc3ac3jsphmgn6lhsdxu",
  "sig": "8d0b7923d26136f7b68780df5122a41782e7cf0c75a641660c2bd8772b5db46b1f9581bdc8fdc48de65d635f346d6949bd6072dda06972378b72b80e884f41d2",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreibwt7cb7li23lkurpsknnzuxu7ligdloqotq5cp2oygeseaf6ak24"
  ],
  "rev": "223mt7puqipw6",
  "seq": 90,
  "of": 106,
  "text_len": 619,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgNp/EH60a2tVIvkprc0vT60GGt0HTh0T9OwYkiAL4CtdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGJkZmY1ZTQ0ODk5MmQxNTE2YjYyYWNmMDE0MWIxYmY0MmM1NGYzYzJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLXVnVxk="
}
---

BLOCK-R6-F1: AC-17's HTML-derivative guarantee is presence-only. scripts/check-rfcs-adrs.sh lines 124-128 checks only that the HTML exists, links the source, and mentions MathJax; it never derives or compares the rendering produced by scripts/render-denotational-semantics.py lines 27-159. In a no-local scratch clone, changing the canonical Markdown title to STALE SOURCE SENTINEL while leaving HTML untouched still made DAY_RFC_PUBLICATION_SKIP=1 scripts/check-rfcs-adrs.sh exit 0, and the sentinel was absent from HTML. The shipped gate can therefore certify a stale local derivative, contradicting REQ-17 and AC-17.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic74tliujdal6eapjjwrwygpoijw6zekikmcp5ph2oxirufk43ei4",
  "sig": "e925de36435c26c5d9c509a64a6693037517c1b0ccd09e0a4502903ee6a199522efea9c2cf329fd88afc2cbf441e828aa6cb6bd7175736dc07526beac9208058",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreibwt7cb7li23lkurpsknnzuxu7ligdloqotq5cp2oygeseaf6ak24"
  ],
  "rev": "223mt7pur34at",
  "seq": 91,
  "of": 106,
  "text_len": 579,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgNp/EH60a2tVIvkprc0vT60GGt0HTh0T9OwYkiAL4CtdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGJkZmY1ZTQ0ODk5MmQxNTE2YjYyYWNmMDE0MWIxYmY0MmM1NGYzYzJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLXVwiGc="
}
---

BLOCK-R6-F2: AC-17's exact-address publication guarantee is not checked. scripts/check-rfcs-adrs.sh invokes scripts/check-rfc0-publication.py, whose vector is only rfc/0/publication, but performs no resolution of subject rfc/1/denotational-semantics. In a no-local scratch clone, deleting .claims/rfc/1/denotational-semantics entirely still made DAY_RFC_PUBLICATION_SKIP=1 scripts/check-rfcs-adrs.sh exit 0. Thus the acceptance surface permits the companion publication projection to disappear without failing, contradicting REQ-17 and AC-17's fresh-clone exact-byte requirement.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieeq5tgtuhr4vhrjrcnivet4kfniszhnq4s2ekthtdhjuqag2v25q",
  "sig": "40cf90d8166d0ff37d511f69132f99a7b19c80259cb8d0e1da86931ee8e9c0a17a44abcf0a3e46550676f7054272cc7ac416f01459cd4ba4c58d0053e0b7535d",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "observation",
  "cites": [
    "bafyreibwt7cb7li23lkurpsknnzuxu7ligdloqotq5cp2oygeseaf6ak24"
  ],
  "rev": "223mt7purnnh4",
  "seq": 92,
  "of": 106,
  "text_len": 370,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgNp/EH60a2tVIvkprc0vT60GGt0HTh0T9OwYkiAL4CtdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGJkZmY1ZTQ0ODk5MmQxNTE2YjYyYWNmMDE0MWIxYmY0MmM1NGYzYzJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLXV5zTE="
}
---

BLOCK-R6-F3: cargo test --workspace is red on the reviewed HEAD. tests/harness_honesty.rs::every_commit_is_accounted_for_under_the_demonstration_rule reports 4 unaccounted commits in span 5d6c0cc..HEAD: a6d40fc, 7a666ab, a55b209, and 3f9ae39. Build, clippy -D warnings, fmt check, and scripts/check-rfcs-adrs.sh --self-test pass, but the project-wide test gate does not.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidde5uzabcfilmi7i7xu4obaqqtzkuw5kvpujctqjf34dcq7yb23u",
  "sig": "4f49a1270a6d3bca61ab19dca124d3c93a855d514b898db1d698904e7aea159a1bb6a825cc9b7831a530caa72f7064713baf5ead9f96fb651c023bab5e7a72be",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt7puuhss2",
  "seq": 93,
  "of": 106,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlZ0Jsb2NrZWRlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChiZGZmNWU0NDg5OTJkMTUxNmI2MmFjZjAxNDFiMWJmNDJjNTRmM2MyaXdvcmtzcGFjZaFpV29ya3NwYWNleEA2NjAyZmZmZTM5NzJjODM4M2NjMTZkZmY3MzdiYWRhMjkxNWNmMmU0ZTk4Yjk4Y2Q5NTQ3MGJiNjBiZGFhMTcza3JlY29yZGVkX2F0GwAGWS11puKT"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreia7sla2lfv2nx2fwxj2dx4mrpafib5v6p7qnrpbfng7u55leefwb4",
  "sig": "a94d8d92808632b8d00c8da2bac9c8f9054fd522517c015cddbb625cd354193c32a51cb15a6c9503ca4fdceb79814084071bb6a7d71b0857d29ef10426524f57",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreibplybwzfhjsjozexsiwjs44wik3wh265hc3ac3jsphmgn6lhsdxu"
  ],
  "rev": "223mt7qp5g4nu",
  "seq": 94,
  "of": 106,
  "text_len": 198,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgL14DbJTpkl2SXkiyZc5ZCt2Pr3Ti2AW0yedhm+WeQ71mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDE2Nzg5ZTkzNDM5NDRkMDBkNThhYmE3ZDkyNzlhNTIzNWQ4MDcwZDKhZkZpbGVBdIJ4KHNjcmlwdHMvcmVuZGVyLWRlbm90YXRpb25hbC1zZW1hbnRpY3MucHl4KDE2Nzg5ZTkzNDM5NDRkMDBkNThhYmE3ZDkyNzlhNTIzNWQ4MDcwZDJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLao2Cfo="
}
---

Correction for AC-17 HTML freshness: the RFC validator now compares the committed derivative against an in-memory rendering of the canonical Markdown, and a source-only hostile mutation is rejected.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihp5pxyrsystpud6k4oxntcwz4qagjzcsxkmlhoh7glx4ya3ksj5y",
  "sig": "d6279e23d1595fae3c3402fd2c15345d2b12f6ede20bdc345dc2d53190dca6e1787eaf027ced92c1fa318eb801e0ca715096438cfe4afd4a0c92c38f3c01ecf6",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreic74tliujdal6eapjjwrwygpoijw6zekikmcp5ph2oxirufk43ei4"
  ],
  "rev": "223mt7qp5zakc",
  "seq": 95,
  "of": 106,
  "text_len": 261,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgX+TWiiRgX4gHpTaNsGe5CbeyRSFME/rz6ddEaFVzZEdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDE2Nzg5ZTkzNDM5NDRkMDBkNThhYmE3ZDkyNzlhNTIzNWQ4MDcwZDKhZkZpbGVBdIJ4LnNjcmlwdHMvY2hlY2stcmZjMS1kZW5vdGF0aW9uYWwtcHVibGljYXRpb24ucHl4KDE2Nzg5ZTkzNDM5NDRkMDBkNThhYmE3ZDkyNzlhNTIzNWQ4MDcwZDJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLao/mZY="
}
---

Correction for AC-17 publication resolution: a real fresh-clone checker verifies the companion Decision CID, repository, FileAt commit/path, source hash, projection presence, and Publication claim; its self-test mutates each coordinate and hides the projection.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibqwwi5i7h4gxivoeyurs5r36knclodjxf5stu6abt7zredyzqonq",
  "sig": "1fd5a93ae812aa3fccb835c88149196dc0b517fdebedafc91600097e2e3236f60824c47a69afcf900b2b05e37b999672676d71b879280acf0f6b425629e340e0",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreieeq5tgtuhr4vhrjrcnivet4kfniszhnq4s2ekthtdhjuqag2v25q"
  ],
  "rev": "223mt7qp6m5ty",
  "seq": 96,
  "of": 106,
  "text_len": 244,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghIdmadDx5U8UxE1FST4orUSydsOS0RUzzGdNIANquuxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOCoWZDb21taXR4KDE2Nzg5ZTkzNDM5NDRkMDBkNThhYmE3ZDkyNzlhNTIzNWQ4MDcwZDKhZkZpbGVBdIJ4GHRlc3RzL2hhcm5lc3NfaG9uZXN0eS5yc3goMTY3ODllOTM0Mzk0NGQwMGQ1OGFiYTdkOTI3OWE1MjM1ZDgwNzBkMml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlktqkkO0g=="
}
---

Correction for the red workspace suite: the four historical v0.13 design/projection commits are accounted append-only with narrow reasons, the fix commit is revert-demonstrated, and the demonstration census now reports zero unaccounted commits.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidhhxwfeci3i4ak6nhmmhpto7jfy2cqo55kjmmv7x534q6snh5poy",
  "sig": "93e4a410f538de9c4cb80df27f830dd97252f9c13b951a20a82434b89e2b7757211ef44c9fa75edd36abeb34070588ab9245435c89a7b25e33bed3c507051e6a",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7qp77gyn",
  "seq": 97,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDE2Nzg5ZTkzNDM5NDRkMDBkNThhYmE3ZDkyNzlhNTIzNWQ4MDcwZDJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLapSs2k="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreia5432iscxppmmcrqrhk6tetr6sxm6fzviolbebc2af43npklwr5i",
  "sig": "23c545095eb76685c87f44291a70e258de3d2a36356a6ceabc0494bec2c67f455a506a75f8fd7aa0347753f38fdbc55866d09f34589f299ba78c9a0ba67215f3",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [
    "bafyreidkes5xyt4raykb4l7avc3henb5wiwus3i2alc3h5bfm5qx573guq"
  ],
  "rev": "223mt7r665uue",
  "seq": 98,
  "of": 106,
  "text_len": 288,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgaiS7fE+RBhQeL+CotnI0PbItSW0aAsWz9CVnYX7/ZqRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGM0ZWM1YjE2NzQ3MWI5OTRjNDg5MGE0ODVhMmQ1MWNmNzhlZDdjMzRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLchB6t8="
}
---

adversarial review of day-rfc-foundations: APPROVE — Fix round closes all three cited BLOCK findings: stale HTML and every publication-address mutation are rejected, the trailer re-derives, the census is 1 demonstrated / 31 exempt / 0 unaccounted, and the full workspace suite is green.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie5bq56rm74hta5anmb2b4hpcfvv54nwjet7nmwfrrjmfh2zfmase",
  "sig": "63396965430734c5465ae168501ddcbfb6af43a3dd3ba9b5b0aa1d8a3a1cf088116602d979283b733b5bff3f1f46ff277c5ae8f444fac6e569a4d74bdf40eb27",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "status",
  "cites": [],
  "rev": "223mt7r66r6e5",
  "seq": 99,
  "of": 106,
  "content": "p2Rib2R5oWZTdGF0dXOhZXZhbHVlaFJlc29sdmVkZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHNkYXktcmZjLWZvdW5kYXRpb25zaWFydGlmYWN0c4GhZkNvbW1pdHgoYzRlYzViMTY3NDcxYjk5NGM0ODkwYTQ4NWEyZDUxY2Y3OGVkN2MzNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlktyEuQ2A=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifvvnxi54inmab7zdqjlzvkq4lacsfhfr66atx5kwam3gqi6z3ydq",
  "sig": "364065d53e52d2f9ade26482758ddabb321725266a889080360052df97a100516ef52ca0b4c4c94038e7c3638f24e4f2909d9499fd0f7a5bb2be2a2632d1308f",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7r6s2lxj",
  "seq": 100,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KGM0ZWM1YjE2NzQ3MWI5OTRjNDg5MGE0ODVhMmQ1MWNmNzhlZDdjMzRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLcmAR0Q="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiczsahqrgnk5l2rnlyp3d3alqy3eg5xorl7w7mfyfcbz3khxnao6i",
  "sig": "18e58be0e2a59728a90480e6c9a3022b5b2754f3696d54e66185e342bbddcce0746307c7c41a4403607d502776e8716911f11ee4785e250d181454821c493bfb",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mt7s3kkuyd",
  "seq": 101,
  "of": 106,
  "text_len": 447,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZWd2Vk1HcHVzU2tucGR0SDRUVjc4eHpVUUZ2bnBtQ1h3MUttbWdWMXloa3duZWFnZW509mdzdWJqZWN0oWVMb2NhbHNkYXktcmZjLWZvdW5kYXRpb25zaWFydGlmYWN0c4KhZkNvbW1pdHgoM2Q1MDVlN2RjOGI2N2M0MjlhZWVjMTViODc1MjBiZjUwOWUyNDA4OKFmRmlsZUF0gnggcmZjcy8xL2Rlbm90YXRpb25hbC1zZW1hbnRpY3MubWR4KDNkNTA1ZTdkYzhiNjdjNDI5YWVlYzE1Yjg3NTIwYmY1MDllMjQwODhpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLgMIZ08="
}
---

The denotational companion now defines telos-relative epistemic subcategories, Grothendieck topologies, a realization prestack, stack-valued descent, schematic obstruction classes, and the attainability envelope of a versioned cooperating-agent system. It distinguishes fiberwise state equivalences from topology-induced local equivalence and treats ordinary cohomology as an abelian shadow of possible nonabelian, stacky, or generalized theories.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiatgwz6aiobbymj33eg4akk2r365ock6swbcsbq6ygwoxpr6uanra",
  "sig": "89d77d91857d13ae2b815d291d2b0a9db03881253c99d69382a27670aef1323a413f6fc6b80b4e7e4f12d360105464ae0ec73f20a5ae01de82c9f3925e1ac76b",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7s3l6pwy",
  "seq": 102,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDNkNTA1ZTdkYzhiNjdjNDI5YWVlYzE1Yjg3NTIwYmY1MDllMjQwODhpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLgMSVzA="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihmbduzpsis5tynvr7x6p5zbzcxg5uh5g5ro4js7kcjqfyk3kt35i",
  "sig": "42c6fcf43357abca992417c401996e5ab0823fe35d26e864229a60470bf637a40496f32013d38e4cb3dc10c5b8db2b3aaea1899f97fe2103bfc339a04204a769",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "decision",
  "cites": [],
  "rev": "223mt7snkmhxb",
  "seq": 103,
  "of": 106,
  "text_len": 408,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgqFmQ29tbWl0eCgwNTM3MjRmMTg0Yzc3MDhlNTIzOWUxMDk0NGFhYTY5MzMyMDFjZTY5oWZGaWxlQXSCeCByZmNzLzEvZGVub3RhdGlvbmFsLXNlbWFudGljcy5tZHgoMDUzNzI0ZjE4NGM3NzA4ZTUyMzllMTA5NDRhYWE2OTMzMjAxY2U2OWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkuJwk3RA=="
}
---

Day's maximal theoretical role is a conservative calculus of collective agency: within explicitly named effective fragments it should compile teloi to witness-bearing realizations or certified obstructions, while exposing unknown whenever the fragment proves neither. Capability and incapability claims must carry their component-level provability bounds rather than treating search failure as impossibility.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigdqajyglnevqj46ywzgtb43l2rtmlvh3xqdqyz2qjllv6hmqmy5a",
  "sig": "56ba3340a46fbc4330613911f0af02c0b446e60475a49c50b75cf7e704fe37853c84711c970b4a525c41d36b623aad2866b111c344e159b965aba6f565a7e7f4",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "result",
  "cites": [
    "bafyreihmbduzpsis5tynvr7x6p5zbzcxg5uh5g5ro4js7kcjqfyk3kt35i"
  ],
  "rev": "223mt7snpn6g7",
  "seq": 104,
  "of": 106,
  "text_len": 361,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIOwI6ZfJEuzw2sf38/uQ5Fc3aH6bsXcTL6hJgXCtqnvqZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlZ3ZWTUdwdXNTa25wZHRINFRWNzh4elVRRnZucG1DWHcxS21tZ1YxeWhrd25lYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2RheS1yZmMtZm91bmRhdGlvbnNpYXJ0aWZhY3RzgqFmQ29tbWl0eCgwNTM3MjRmMTg0Yzc3MDhlNTIzOWUxMDk0NGFhYTY5MzMyMDFjZTY5oWZGaWxlQXSCeCByZmNzLzEvZGVub3RhdGlvbmFsLXNlbWFudGljcy5tZHgoMDUzNzI0ZjE4NGM3NzA4ZTUyMzllMTA5NDRhYWE2OTMzMjAxY2U2OWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhANjYwMmZmZmUzOTcyYzgzODNjYzE2ZGZmNzM3YmFkYTI5MTVjZjJlNGU5OGI5OGNkOTU0NzBiYjYwYmRhYTE3M2tyZWNvcmRlZF9hdBsABlkuJ1mRGA=="
}
---

The mathematical arc of Draft RFC 1 now reaches its intended specification target: indexed process equipment semantics, telos-relative sites and obstruction theory, effective realization fragments, a component provability ledger, and the constructive capability-compiler horizon are summarized in the canonical denotational companion and rendered HTML artifact.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidsxnsrygwam3fhbdj4yqsypdw4fachwemkrqthn7di6xhf7gz2z4",
  "sig": "ae109f9db1fa4cf49bbf29bc7aca75878332f19c1ec4e3fb82aadb3813efeb741648f59c3be23aa5977a39ccfb9ab0547cad3e3e5313d3ccffa9abec51f5ca79",
  "author": "did:key:zDnaegvVMGpusSknpdtH4TV78xzUQFvnpmCXw1KmmgV1yhkwn",
  "subject": {
    "local": "day-rfc-foundations"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mt7snqahjt",
  "seq": 105,
  "of": 106,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVndlZNR3B1c1NrbnBkdEg0VFY3OHh6VVFGdm5wbUNYdzFLbW1nVjF5aGt3bmVhZ2VudPZnc3ViamVjdKFlTG9jYWxzZGF5LXJmYy1mb3VuZGF0aW9uc2lhcnRpZmFjdHOBoWZDb21taXR4KDA1MzcyNGYxODRjNzcwOGU1MjM5ZTEwOTQ0YWFhNjkzMzIwMWNlNjlpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDY2MDJmZmZlMzk3MmM4MzgzY2MxNmRmZjczN2JhZGEyOTE1Y2YyZTRlOThiOThjZDk1NDcwYmI2MGJkYWExNzNrcmVjb3JkZWRfYXQbAAZZLidjNZA="
}
---
