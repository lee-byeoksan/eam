# F01–F07 요구사항 대조 — 0.8.11

대상은 feature-candidates.md의 F01–F07 및 도입 원칙이다. "당장 우선하지 않는 후보"는
이 구현 목표에 포함하지 않는다. 각 기능의 자동 시험, 직접 GUI 관찰, 사용자 보고를 구분한다.

## 산출물 일치

- 설치본: var/package-profile-0.8.11/elpa/emacs-ai-0.8.11/.
- tar SHA-256: 80b4cfe5e45b60c17e00b0f40d02faf7f7ef84bd37f3dfd4a46b873e9fae7f95.
- 설치된 모든 manifest 파일의 해시와 현재 제품 Lisp 소스의 일치를 확인했다.
- 최종 재대조: manifest 68개, 제품 Lisp 13개, 근거 링크 22개 확인
  ([산출물 재대조](evidence/gui-087/final-artifact-audit.json)). 제품 수정이나 새 실행 시험은 아니다.
- [설치·PTY·Magit 60개 ERT, 별도 프로세스 복원, 제거 후 기록 보존](evidence/gui-087/package-install-0811.log).
- [산출물 범위](evidence/gui-087/package-0811.json). 테스트 개수를 GUI 통과 근거로 사용하지 않는다.

## 원문 기능별 근거

| 기능 | 원문 요구사항과 확인된 근거 |
| --- | --- |
| F01 | CLI·프로젝트·브랜치·프로세스 표시, 프로젝트별 조회, 동시 8개·2초 제한과 취소는 app-test.el 및 설치 회귀에서 확인. [100개 프로젝트 측정](evidence/session-selector/bounded-parallel.json). GUI 단일 후보 선택·복귀와 사용자 확인의 긴 한글 세 후보·브랜치 구분·Tab completion·취소. 일반 버퍼 물리 한글 조합·undo는 사용자 직접 확인. |
| F02 | 공식 훅/OSC 이벤트를 수신하며 바이트 수로 작업 상태를 추정하지 않음. notifications-test.el에서 귀속·중복·종료 버퍼·개수/문자 제한·시작/재개 옵션·추가 전송 없음 검증. [두 실제 CLI 승인 이벤트](evidence/notifications/live-approval.json). 사용자 GUI 읽음 전환·목록 복귀·다음 미읽음 이동 통과. 선택적 데스크톱 전달은 기본 꺼짐, 원본 버퍼·중복·백엔드 실패 처리 테스트 통과. |
| F03 | 선택한 diff만 경로·줄 위치·의견과 함께 편집 가능한 초안에 추가. review-test.el의 한글 경로·여러 파일/hunk·삭제 줄·분량 제한·취소·undo·미전송 검증. 실제 Magit rename/접힘 테스트와 [GUI 초안](evidence/gui-087/magit-draft.png), [undo](evidence/gui-087/magit-draft-undo.png), [입력 저널 0바이트](evidence/gui-087/magit-input-audit.json). 최종 전송은 기존 명시적 transfer 명령이며 자동 제출 없음. |
| F04 | 실제 Git 생성·기존 경로 선택·격리·한글/공백·dirty/ignored/프로세스 삭제 보호는 worktree-test.el 및 지속 세션 통합 시험. [실제 두 CLI cwd와 삭제 보호](evidence/worktrees/live-start.json). [0.8.10 GUI ~ 경로 생성 후 Git 대조](evidence/gui-087/worktree-created-0810.json). 파일 탐색 GUI도 확인. |
| F05 | 명령 등록은 데이터 저장만 하며 명시적 실행에서 cwd 분리. tasks-test.el의 종료 코드·취소·전체 기록·버퍼/대기 상한·undo 꺼짐·재실행·기록 실패·프로젝트 격리 검증. 실제 HTTP 서버 시험 및 GUI 등록·실행·오류 이동·이전 기록·스크롤·취소·재실행 확인. [로그 대조](evidence/gui-087/task-log-audit.json). |
| F06 | 프로젝트·세션 식별자·창 배치 저장/복원, 자동 CLI/파일 실행 없음, 사라진 경로·중복·형식 변경·실패 롤백은 workspace-test.el와 workspace-fresh.py. [안내 첫 줄 수정 GUI](evidence/gui-087/workspace-fixed-088.png). 사용자 확인으로 새 GUI에서 안내 복원 후 r로 명시적 Codex 재접속 통과. |
| F07 | 소유 토큰과 전용 서버, 명시적 종료, 재접속 시 기록 중복 방지, 제한된 PTY/알림 메모리, 실패 시 미확인 상태 유지. manager/recorder/events 및 설치 통합 시험, [세 장애 시험](evidence/persistence/failures-0.8.6.log), [600초 측정](evidence/persistence/soak-600s-summary.json). [Emacs 종료 전후 동일 CLI PID·새 GUI 연결·정리](evidence/gui-087/persistent02-lifecycle.json). 실제 Claude/Codex 대화 ID 재개 및 편집기 왕복은 별도 CLI 증거, Codex 수동 GUI 분리·복원·미제출 입력·편집기 반환·종료는 사용자 직접 확인. |

GUI 및 사용자 보고의 정확한 단계는 [관찰 기록](evidence/gui-087/observations.md)에 있다.

## 도입 원칙과 공통 검증

제품은 기존 CLI를 실행하고 로컬 Git/JSON/PTY 정보를 다룬다. 사용자 입력을 초안에서
확인하고 명시적으로 전송한다. 시스템 프롬프트·스킬·컨텍스트 자동 삽입, AI 상태 분류,
자동 요약·병렬 AI 배포를 추가하지 않았다. Claude 훅은 알림용 명령이며 개인 설정을 쓰지 않는다.
원본 Ghostel은 고정된 의존성으로 사용하며 다른 앱의 포크를 제품 기반으로 삼지 않았다.

M-x 명령과 키 참조표를 제공한다. [명령 이름 대조](evidence/gui-087/manual-command-audit.json)는
문서 수록 검사이며 동작 시험과 구분한다. 개인 init을 읽지 않는 별도 프로필을 사용한다.
전체 기록은 디스크, 화면과 이전 페이지는 제한된 분량, 출력 갱신은 묶음 처리, 출력 undo는
꺼지고 사용자 입력 undo는 유지된다. [GUI 스트림 측정](evidence/gui-087/scroll-summary.json)과
사용자의 직접 한글 조합·수정·undo 확인은 배치 부하 측정과 별도다.

## 검증 범위와 유지할 한계

- 한글 물리 입력은 사용자가 일반 입력 버퍼와 리뷰 의견 미니버퍼에서 확인했다.
  세션 선택도 사용자 설명으로 한글 입력·수정이 정상임을 확인했다
  ([보고](evidence/gui-087/completion-ime-user.json)). Space는 기본 단어 완성 동작이다.
- 선택적 macOS 배너는 별도 alert/osx-notifier 시험에서 사용자 직접 표시 확인을
  마쳤다([근거](evidence/gui-087/desktop-probe-user.json)). 기본 설치본에는 alert가
  포함되지 않으므로 기본 상태의 Emacs 메시지 대체 동작과 구분한다.
- gui-created의 Claude 신뢰는 사용자 명시 승인 후 선택했고 실제 경로와 빈 입력
  화면을 확인했다([화면](evidence/gui-087/worktree-claude-trusted.png)). Codex도
  사용자 절차 보고와 후속 [실제 화면](evidence/gui-087/worktree-codex-user-confirmed.png)으로
  해당 경로의 입력 화면을 확인했다. 두 프로세스의 cwd 대조·종료와 Git 변경 없음도
  확인했다([정리](evidence/gui-087/worktree-final-cleanup.json)). AI 작업 요청은 제출하지 않았다.
- Claude도 Computer Use로 실제 GUI 편집·분리·새 0.8.11 GUI의 명시적 attach·
  편집기 반환·종료를 확인했다([기록](evidence/gui-087/observations.md)).
  물리 한글 조합이나 사용자 키보드로 Claude 전체 흐름을 확인한 결과는 아니다.
- 서버 소실 후 임의로 분리된 자식의 종료를 증명하거나 삭제 보호를 강제로 해제하는 기능은
  지원하지 않는다. 종료 미확인 상태 보존과 삭제 거절이 현재의 실패 처리다.
- 600초 측정은 무제한 장기 운용·메모리 누수 부재의 증명이 아니다. 도구 키 전달 불안정도 남는다.

위 한계는 배포 안내에 유지한다. 모든 입력기/프런트엔드 조합, 임의 daemon의
자동 복구를 완료했다고 주장하지 않는다. F01–F07의 구현과 명시된 핵심 동작 검증 근거는
확보됐으며, 사용자 시험용 0.8.11 산출물을 제공한다.

## 전체 완료 판정

F01–F07의 구현 및 명시된 검증 기준을 충족했다. 마지막 미확인 항목이던 세션 선택의
물리 한글 입력·수정은 사용자가 기존 관찰을 명확히 하여 정상으로 확인했다. Space
미입력은 기본 completion 동작이며 제품 결함이 아니다. 재시험 없이 근거를 보완했다.

각 기능의 자동 시험·실제 CLI·GUI 및 사용자 보고는 위 표와 관찰 기록에 연결했다.
최종 설치본 manifest, 제품 Lisp 및 Python bridge 소스 일치도 확인했다. 이 목표의
필수 잔여 작업은 없다. 위에 명시한 장시간 운용, 입력기별 조합 및 서버 소실 후
임의 자식 종료 증명 등의 지원 한계는 완료 판정과 함께 유지한다.

사용자 시험 산출물은 0.8.11이다. tar는 불변으로 유지하며, 빌드 이후 추가한 GUI
근거와 완료 판정은 작업 폴더의 문서를 기준으로 한다. 이 후속 기록이 기존 tar에
포함되어 있다고 주장하지 않는다.
