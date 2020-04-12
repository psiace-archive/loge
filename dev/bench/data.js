window.BENCHMARK_DATA = {
  "lastUpdate": 1586711399350,
  "repoUrl": "https://github.com/PsiACE/loge",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "psiace@outlook.com",
            "name": "Chojan Shang",
            "username": "PsiACE"
          },
          "committer": {
            "email": "psiace@outlook.com",
            "name": "Chojan Shang",
            "username": "PsiACE"
          },
          "distinct": true,
          "id": "52972b3a62fc1e68c68a7812e49b1c7d9e66d018",
          "message": ":construction_worker: CI for doc, release, benchmark.",
          "timestamp": "2020-04-13T00:44:03+08:00",
          "tree_id": "4fe9db67172ec16892bda0fc678e033fc6de8251",
          "url": "https://github.com/PsiACE/loge/commit/52972b3a62fc1e68c68a7812e49b1c7d9e66d018"
        },
        "date": 1586710612630,
        "tool": "cargo",
        "benches": [
          {
            "name": "b10_no_logger_active",
            "value": 52,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "b20_initialize_logger",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "b30_relevant_logs",
            "value": 265769,
            "range": "± 60961",
            "unit": "ns/iter"
          },
          {
            "name": "b40_suppressed_logs",
            "value": 253123,
            "range": "± 74541",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "psiace@outlook.com",
            "name": "Chojan Shang",
            "username": "PsiACE"
          },
          "committer": {
            "email": "psiace@outlook.com",
            "name": "Chojan Shang",
            "username": "PsiACE"
          },
          "distinct": true,
          "id": "90c5e5c627c24cdf15ca2c26e3a19b2a7f9b8f28",
          "message": ":green_heart: Keep files.",
          "timestamp": "2020-04-13T01:08:45+08:00",
          "tree_id": "ead55207785c985ce2627796b05308e347fb74c2",
          "url": "https://github.com/PsiACE/loge/commit/90c5e5c627c24cdf15ca2c26e3a19b2a7f9b8f28"
        },
        "date": 1586711398332,
        "tool": "cargo",
        "benches": [
          {
            "name": "b10_no_logger_active",
            "value": 51,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "b20_initialize_logger",
            "value": 0,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "b30_relevant_logs",
            "value": 231229,
            "range": "± 26855",
            "unit": "ns/iter"
          },
          {
            "name": "b40_suppressed_logs",
            "value": 215849,
            "range": "± 31661",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}