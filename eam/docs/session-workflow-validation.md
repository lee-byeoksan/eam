# 실제 연결의 세션 전환과 재시작

2026-09-08. 사용자 요청에 따라 가짜 연결을 선행하지 않고 실제 Claude Code 2.1.263·Codex 0.153.4로 검증했다. 하나의 별도 `Emacs --batch -Q`에서 Ghostel 터미널 두 개를 열었다. 개인 GUI와 개인 설정은 수정하지 않았다.

## 결과

| 검사 | 확인한 결과 |
| --- | --- |
| 기존 대화 재개 | 기존 한글 경로 시험 대화를 각각 native ID로 재개, 이전 명령과 출력 복원 |
| 두 버퍼 전환 | 20회 왕복으로 40번 선택. 각 버퍼의 세션 객체·프로젝트 디렉터리·살아 있는 PTY·앱 메뉴 minor mode 유지 |
| 재개 후 실제 응답 | 각 CLI에 아래 요청 1회씩 제출, 둘 다 `PATH_OK` 응답 |
| 한쪽 종료 격리 | Claude `/exit` → exit 확인 → Close 후에도 Codex run 유지 |
| Emacs 전체 재시작 | Codex `/quit`와 시험 Emacs 종료 후 새 Emacs에서 두 ID 재개. 방금 요청과 응답 복원 |
| 정리 | 두 번째 실행도 `/exit`·`/quit` 후 exit 확인, 시험 Emacs 종료 |

명시적 모델 요청은 Claude 1회·Codex 1회다. 재시작 뒤 추가 모델 요청은 보내지 않았다. 사용한 요청은 “이 대화에서 앞서 실행한 명령의 출력 표식만 답하세요. 파일이나 도구를 사용하지 마세요.”다. 이전 표식을 프롬프트에 다시 넣지 않았다. 응답 화면에서 새 도구 실행은 관찰하지 않았다. 요청 횟수는 제공자 내부 네트워크 호출 횟수나 과금 집계와 동일하지 않다.

Claude는 Fable 5.1 / Claude Max, Codex는 gpt-6-astra를 표시했다. 모델 설정을 변경하지 않았다. Codex는 첫 일반 요청 뒤 세션 제목 변경 안내를 자체 표시했다. 앱의 제목 생성 호출은 아니다.

[Claude 응답](cli-validation-evidence/session-workflow/claude-response.txt), [Codex 응답](cli-validation-evidence/session-workflow/codex-response.txt), [새 Emacs의 Claude](cli-validation-evidence/session-workflow/claude-restarted.txt), [새 Emacs의 Codex](cli-validation-evidence/session-workflow/codex-restarted.txt).

## 검증 중 구분한 문제와 한계

- 처음 검증 제어에서 키 이름을 `enter`로 보내 제출되지 않았다. 제품 명령이 사용하는 Ghostel 키 이름 `return`으로 바로잡았다. 확인 요청을 중복 제출하지 않았다.
- 선택되지 않은 Codex 버퍼의 화면 덤프가 오래된 상태를 보였다. 원시 ANSI 기록에는 요청·응답이 있었고 해당 버퍼를 선택한 뒤 화면에도 나타났다. 숨겨진 버퍼의 문자열만 보고 CLI 정지나 응답 누락으로 판단하지 않는다. GUI에서 실제 전환할 때의 지연 시간은 이번에 측정하지 않았다.
- 두 제공자의 이름이 다른 세션을 시험했다. 같은 제공자·여러 프로젝트의 기본 버퍼 이름이 충분히 구별되는지는 아직 별도 사용성 검토가 필요하다.
- 재개는 이미 알고 있는 ID를 사용했다. 사용자가 목록에서 원하는 대화를 찾아내는 편의성을 입증한 결과는 아니다. 앞선 목록 전환 검사는 별도 기록에 있다.
- 물리 키보드 IME, GUI 마우스 동작, 강제 종료 시 하위 작업 정리는 이번 범위가 아니다.

실행 스크립트 [workflow.el](cli-validation-evidence/session-workflow/workflow.el)은 이번 두 시험 ID와 작업 경로가 고정된 증거다. 자동 회귀 테스트에 포함하지 않는다. 재실행하면 실제 계정 CLI를 시작하므로 같은 대화를 다른 CLI에서 쓰고 있지 않은지 먼저 확인해야 한다. 원시 기록과 제어 스크립트는 `var/cli-validation/20260908-session-workflow/` 및 `var/terminal-*.ansi`에 있다.
