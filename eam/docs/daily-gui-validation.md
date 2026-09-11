# 최신 일상 앱 GUI 확인

2026-09-09 G01/G10/G13 부분 검증. 기존 local.emacs-ai.app은 실행 중이지만 Computer Use가 cgWindowNotFound를 반환했다. 기존 창을 종료하지 않고 `python3 scripts/prepare-gui-test.py --app --instance review`로 동일한 제품 진입 코드의 별도 앱 ID를 준비했다. 준비 자체는 GUI를 열지 않으며 기존 설치 Emacs를 사용한다. Computer Use로 `var/Emacs AI review.app`을 열었다. 개인 Emacs·기존 앱은 변경하지 않았다.

| GUI 조작 | 실제 관찰 |
| --- | --- |
| 일상 시작 화면 | 최신 버튼·안내 표시, 처음에 단일 창, CLI 미실행 |
| Caffeine 버튼 클릭 | 활성화 메시지와 모드라인 Caffeine 표시 |
| Caffeine 다시 클릭 | 비활성화 메시지와 모드라인 표시 제거 |
| Claude 시작 버튼 | macOS 폴더 패널 대신 Project directory 미니버퍼 열림 |
| 경로 type_text | 영문자가 누락돼 `/-20260909///`만 추가됨. 선택하지 않음 |
| Ctrl-A/Ctrl-K와 붙여넣기 | 경로가 지워지지 않고 s-l is undefined 표시, clipboard read 시간 초과. 정상 전달로 세지 않음 |
| Minibuf → Quit | 메뉴 클릭으로 입력 취소·시작 화면 복귀 |
| Claude 시작 → 기본 프로젝트 → Minibuf → Enter | 실제 Claude 터미널·프로젝트 헤더·빈 입력란 표시 |
| Terminal AI → Interrupt | CLI의 Press Ctrl-C again to exit 안내 표시 |
| Terminal AI → Close experiment | 요청·작업 없는 시험 터미널 닫힘, 시작 화면 복귀. /exit 정상 종료 검사는 아님 |

증거: [Caffeine 켜짐](cli-validation-evidence/daily-gui/caffeine-gui-on.png), [꺼짐](cli-validation-evidence/daily-gui/caffeine-gui-off.png), [실제 Claude 시작](cli-validation-evidence/daily-gui/daily-claude-gui.png).

실제 GUI는 Claude **2.1.265**, Fable 5.1, native auto mode를 표시했다. 앱이 권한 모드를 지정하지 않아 CLI 기본값이 적용된 것이다. 앞선 수동 승인·개발·goal 검사의 **2.1.263**과 구분한다. 이번에는 모델 프롬프트를 보내지 않았고, 네이티브 CLI 내부 네트워크 호출 유무는 계측하지 않았다.

G13의 GUI 토글·표시는 확인됐으며 OS assertion 수명 검사는 [별도 결과](caffeine.md)를 따른다. 최신 GUI의 Codex 시작·직접 문자열 입력·재개·외부 편집 왕복·검색·스크롤/부하 반응은 아직 미완료다. 이번 도구 입력 실패를 물리 키보드나 Emacs의 일반 입력 결함으로 단정하지 않는다. 현재 review 앱은 시작 화면이고 Caffeine은 꺼져 있다. 기존 앱의 폴더 이동창 문제는 별도 상태로 남는다.

## Codex GUI 후속

같은 review 앱에서 Codex 시작 버튼→기본 프로젝트→Minibuf Enter로 실제 Codex 0.153.4를 시작했다. 초기 MCP 로딩 뒤 gpt-6-astra xhigh·프로젝트 경로·빈 입력란을 확인했다. [시작 화면](cli-validation-evidence/daily-gui/daily-codex.png).

창의 zoom 동작 후 세로 높이와 터미널 표시 영역이 늘어났고, 위로 스크롤하자 스크롤바가 위쪽으로 이동하면서 시작 출력 위치가 변했다. [확대·스크롤 화면](cli-validation-evidence/daily-gui/codex-zoom-scroll.png). 적은 시작 출력의 육안 관찰이며 긴 로그 부하나 입력 지연 수치의 검증은 아니다. 직접 문자열 입력·재개·외부 편집 왕복·검색·장시간 부하 반응은 계속 미완료다.

모델 프롬프트를 제출하지 않았으며 확인 후 Terminal AI의 Close experiment로 해당 시험 PTY를 닫았다. 네이티브 /quit 정상 종료 검사로 세지 않는다. 시작 화면이 다시 보이고 Caffeine은 꺼진 상태다. 기존 개인/시험 앱의 프로세스는 조작하지 않았다.

## 장시간 배치 완료 후 단일 키 대조

동일 review GUI에서 Codex 버튼으로 일반 프로젝트 미니버퍼를 열고 Computer Use `press_key(key="a")`를 단 한 번 보냈다. AX 값은 `Project directory: ~/workspace/emacs-ai/ᆼ`이 됐다. 잘못된 경로는 Minibuf → Quit으로 취소했고 CLI를 실행하지 않았다. Help의 중첩 Describe Key 항목을 직접 클릭해 진단을 시도했으나 키 설명 화면은 얻지 못했다. 이어 보낸 `a`는 시작 화면에서 자모가 undefined라는 에코 메시지를 만들었다. [스크린샷](cli-validation-evidence/daily-gui/single-key-a-jamo.png).

새 증거는 변환이 Ghostel·CLI 이전의 일반 Emacs 입력에서도 발생한다는 것이다. macOS 입력 소스/키보드 레이아웃과 Computer Use 이벤트 전달 중 어느 층이 원인인지는 이 관찰만으로 분리되지 않는다. 앞선 영문자 누락을 단순히 도구가 키를 버리는 현상으로 확정하지 않는다. 물리 키보드의 한영 전환·조합 결과는 여전히 별도 확인이 필요하다. 개인 설정이나 OS 입력 소스를 변경하지 않았고 실제 AI 요청도 없었다. review GUI는 시작 화면에 남겼다.

후속 우회 검사: 프로젝트 미니버퍼에 `sky.set_value`로 시험 프로젝트 경로를 지정하려 했으나 `-10005: Cannot set a value for an element that is not settable`로 거절됐다. 경로는 바뀌지 않았고 Minibuf → Quit으로 취소했다. 따라서 AX 값 설정으로 입력 경로를 우회할 수 없었다.

읽기 전용 `defaults read com.apple.HIToolbox AppleCurrentKeyboardLayoutInputSourceID` 결과는 `com.apple.keylayout.390Hangul`, `AppleSelectedInputSources`의 Input Mode는 `com.apple.inputmethod.Korean.390Sebulshik`이었다. 저장된 macOS 입력 설정은 세벌식 390을 가리킨다. 이는 앱별 실시간 입력 소스 API 계측이 아니며, 앞선 변환 관찰과 함께 영문 입력 소스 대조가 필요하다는 근거다. 설정을 쓰거나 다른 입력기를 설치하지 않았다.

## ABC 대조와 개별 키 전달 성공

후속 읽기 전용 조회에서 입력 레이아웃이 `com.apple.keylayout.ABC`로 바뀐 것을 확인했다. 동일 `press_key("a")`는 일반 프로젝트 미니버퍼에 정확히 `a`를 입력했다. 반면 ABC에서도 `type_text("var/workflow-20260909/codex/workspace/")`는 영문자가 빠진 경로가 됐다. 잘못된 경로를 취소하고, 새 미니버퍼에서 각 문자를 `press_key`로 순차 전송(slash/minus 키 이름 사용)하자 `~/workspace/emacs-ai/var/workflow-20260909/codex/workspace/` 전체가 정확히 입력됐다. Return도 정상 전달돼 실제 Codex가 시작됐다. 따라서 개별 키 방식으로 GUI 경로 입력을 진행할 수 있다. 이 결과는 물리 IME 조합 검증이 아니다.

Codex는 상위 시험 디렉터리 `/Users/lee.byeoksan/workspace/emacs-ai/var/workflow-20260909/codex/`의 신뢰 확인을 표시했다. 해당 디렉터리에서 열거된 파일은 기존 검증 코드와 증거였다. Yes 선택의 Return 입력은 자동 승인 검토에서 거절됐다. 이유는 프로젝트 설정·훅·실행 정책을 신뢰하는 별도 보안 경계 변경에 대한 구체적 사용자 승인이 없다는 것이었다. 재시도나 우회하지 않았으며, 실제 모델 프롬프트는 보내지 않았다. review 창의 Codex 신뢰 확인 화면에서 사용자 승인을 기다린다.
