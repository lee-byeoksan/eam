# Codex 다중 프로젝트·초안 식별

2026-09-09, G02. 실제 Codex 0.153.4를 같은 배치 Emacs의 Ghostel 터미널 세 개에서 실행했다. 기존 시험 대화를 `workspace`와 `한글 프로젝트`에서 각각 재개하고, `workspace`에는 별도의 새 대화를 열었다. 같은 대화 ID의 중복 writer는 만들지 않았다.

## 발견과 수정

같은 프로젝트의 터미널은 `<2>`로 구별됐으나 초안 이름은 초안 생성 순서로 번호가 붙었다. 두 번째 터미널의 초안을 먼저 열면 그 초안에 번호가 없고 첫 번째 터미널의 초안에 `<2>`가 붙었다. 실제 입력 대상은 세션 객체로 정확히 연결돼 있지만 사용자가 이름으로 판단하기 어려웠다.

초안 이름을 부모 터미널 이름에서 만들도록 수정했다. 부모의 `<2>` 접미사가 그대로 유지된다. 사용자 지정 터미널 이름에는 `Draft for`와 부모 이름을 표시한다. 변경 전후는 [이전 이름](cli-validation-evidence/codex-projects/drafts-before.txt), [수정된 이름](cli-validation-evidence/codex-projects/drafts-after.txt)에 있다. 빈 초안만 다시 생성해 실제 CLI를 유지한 채 수정 결과를 확인했다.

## 실제 연결 검증

- 앱의 세션 선택 함수를 통해 세 세션을 20회씩, 총 60회 전환하고 선택된 세션 객체를 확인했다. 선택 문자열 입력은 배치 제어기로 지정했으므로 GUI 완성 목록 조작 검사는 아니다.
- 각 초안에 `UNSENT-PROJECT-0`, `1`, `2`를 넣고 제품 전송 명령으로 전달했다. 각각의 실제 CLI 화면에는 자기 입력만 나타났다. Enter는 보내지 않았다.
- Ctrl+A·Ctrl+K로 각 입력을 비운 뒤 화면에서 제거됐음을 확인했다. 첫 CLI를 `/quit`로 종료한 뒤 다른 두 CLI는 계속 실행 중이었다. 앱의 선택 기능으로 종료 버퍼에도 다시 접근했다.
- 나머지 CLI도 `/quit`로 종료해 세 상태 모두 exit를 확인하고 배치 Emacs를 종료했다.
- 명시적 모델 프롬프트 0회. CLI 자체 내부 네트워크 호출 수를 뜻하지 않는다.

[입력 격리 검사](cli-validation-evidence/codex-projects/checks.json), [한쪽 종료](cli-validation-evidence/codex-projects/one-exited.el), [전체 종료](cli-validation-evidence/codex-projects/all-exited.el). 제어 코드와 전체 화면은 `var/cli-validation/20260909-codex-projects/`에 보관한다.

앱 회귀 3개, 터미널 회귀 5개 통과. 초안을 역순으로 여는 회귀 검사를 추가했다. GUI에서 긴 이름·완성 목록의 실제 편의성은 G01에 남는다. X01 전용 목록 화면은 현재 구현하지 않는다. 제공자·프로젝트 이름과 기존 세션 선택으로 이번 흐름을 충족했고, 별도 목록의 필요성은 GUI 관찰 후 다시 판단한다.
