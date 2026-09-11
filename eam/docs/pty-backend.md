# 기본 PTY 백엔드 — 2026-09-16

모든 세션은 Rust PTY 데몬을 사용한다. 백엔드 선택 설정은 없다. 버전은 0.1.0이다.

## 현재 제품 경로

Ghostel → native attach → Unix socket → Rust PTY daemon → 공식 CLI.

Python 실행 의존성을 제거했고, 이전 Python 제품 경로의 별도 recorder PTY도 제거했다. Rust 데몬이 선택적 원시 기록·알림 이벤트·CLI 종료 정리를 직접 처리한다. 빌드·캐시·플랫폼 조건은 [native runtime](native-runtime.md)에 정리했다.

Emacs가 세션 manager와 연결 client를 실행한다. 데몬은 백그라운드에서 CLI를 유지하므로 Emacs를 닫아도 CLI는 계속 실행한다. attach lock과 editor endpoint PID/token 검증을 유지한다. 데몬 상태/종료는 소유 ID로 검증한 별도 Unix datagram socket을 사용하며 저장된 PID에 임의로 신호를 보내지 않는다. CLI의 실제 종료 코드를 기록하고 종료된 세션은 활성 목록에서 제외한다.

## 화면 크기와 출력 제한

attach client가 SIGWINCH를 받아 poll 대기를 깨우고 CLI PTY 크기를 즉시 전달한다. 아무 입력 없는 가짜 CLI의 실제 Ghostel 창 분할→복원으로 너비 전달을 검증한다. 기존 출력 줄바꿈 재구성은 Ghostel/CLI 지원 범위에 따른다.

재연결용 시작 출력은 최대 5MiB, 16KiB 패킷으로 나눠 전송한다. attach 순간 보관 기록 외에 최대 약 5MiB(+패킷 헤더)의 재생 전송 메모리를 일시적으로 사용한다. live 큐 256KiB와 분리되며 느린 연결이 live 큐를 넘으면 연결을 끊고 CLI는 유지한다. Ghostel 스크롤백 5MiB는 별도다.

시작 기록 재생은 정확한 화면 snapshot이 아니다. 상한 초과 시 화면 복원 불가를 알리며 CLI는 유지한다. 연결 없는 동안의 terminal query·크기 변경 이력·복잡한 TUI 복원에는 한계가 있다. 이전 대화는 native history로 조회한다.

## 검증 범위

Rust 가짜 CLI로 UTF-8·붙여넣기·C-g 외부 편집기·C-c C-c 왕복·detach/attach/quit·detached 자연 종료·5MiB 재생 경계·raw 기록 기본 OFF를 검사한다. 기본 스크립트는 `scripts/test-native.sh`, 설치 위치에서의 첫 빌드와 PTY 재검증은 `scripts/test-native-package.sh`다. GUI 프레임·실제 AI 호출은 하지 않는다.

이전 Python 구현의 97→143→61열 signal wakeup pipe 검증과 27개 앱 회귀 등은 당시 결과다. Rust의 측정치나 모든 과거 회귀의 재실행으로 해석하지 않는다. 체감 스크롤·한글 조합·실제 Claude/Codex 장시간 사용은 별도 GUI 검증 대상이다.

## 기존 세션

새 세션부터 native 런타임을 사용한다. 이미 실행 중인 Python 데몬·EDITOR 환경을 강제로 바꾸지 않는다. Python 없는 환경으로 완전히 옮기려면 기존 CLI를 정상 종료한 뒤 새 세션 또는 저장 대화 `eam-resume`으로 재개한다. 사용자 세션은 이 변경 작업에서 종료하지 않았다.
