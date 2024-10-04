# Axum Tutorial Project

> 아래의 유튜브 영상을 보고 따라한 프로젝트 입니다.
> [reference](https://www.youtube.com/watch?v=XZtlD_m59sM&t=3s)

## Commands

**Pre-requisite**
```bash
cargo install cargo-watch
```

**For running backend server**
```bash
cargo watch -q -c -w src/ -x run
```

- `cargo watch` - 파일의 변경 사항을 감지하고, 특정 명령을 자동으로 실행.
- `-q` (--quiet) - 출력을 최소화.
- `-c` (--clear) - 명령 실행 전 화면을 지움.
- `-w` (--watch) - 감시할 디렉토리를 지정.
- `-x` (--exec) - 실행할 명령을 지정(프로젝트에서는 run).

**For running test**
```bash
cargo watch -q -c -w src/ -x tests/ -x "test -q quick_dev -- --nocapture"
```

- `test -q quick_dev -- --nocapture` - 테스트 실행 명령어.
- `-- --nocapture` - 표준 출력을 갭처하지 않고, 그래도 출력하도록 (실시간 캡처).
