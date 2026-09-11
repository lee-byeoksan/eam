# 0.8.6 GUI 관찰 — 2026-09-10

- Computer Use `get_app_state` 연결 복구 확인.
- 전용 앱 ID `local.emacs-ai.app.gui086`, `local.emacs-ai.app.input086`를 준비해
  개인 init 없이 설치본 0.8.6을 로드했다. 원래 개인 Emacs에는 입력하지 않았다.
- 초기 `alt+x` 및 Esc/x 시도는 기대한 M-x 화면으로 진행하지 못했다.
  한 화면에서는 `M-1 is undefined`가 표시됐다. 원인 확정 전이며 Emacs 결함으로 판정하지 않는다.
- 가짜 입력 버퍼에 `type_text`로 한글/영문 혼합 문자열을 보냈을 때 기대 문자열이
  입력되지 않고 Shell command 미니버퍼가 열렸다. Return을 보내지 않고 Esc 3회로 취소했다.
- `paste`는 `Timed out waiting for the application to read the clipboard`로 실패했다.
- 이후 단일 `press_key x`는 정상 x를 입력했다. [화면](key-x.png).
- 입력을 단계별로 나눠 확인했을 때 Esc → x → `type_text emacs-ai-help` → Return은
  실제 M-x 미니버퍼와 0.8.6 사용 안내를 열었다. 메뉴는 사용하지 않았다.
- 사용 안내는 find-file로 열려 편집 가능했다. q 입력 뒤 버퍼가 수정 상태가 됐으며
  저장하지 않았다. 소스의 도움말을 view-file로 수정하고 read-only/q 복귀 ERT를 추가했다.
  앱 테스트 16개 통과. 이 수정은 아직 0.8.6 설치본에 반영되지 않았다.

이 결과는 GUI 연결 및 M-x 도움말 실행의 부분 검증이다. 물리 한글 조합·undo,
전체 F01–F07 GUI 흐름, 지속 출력 중 스크롤은 아직 통과 판정하지 않는다.
두 전용 시험 창은 후속 검증을 위해 열어 두었다. 새 AI 요청은 제출하지 않았다.
