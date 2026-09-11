# 연결·실행 오류 복구

2026-09-09, G09 진행 중. 앱은 CLI 오류를 원시 PTY 기록에 보관하고 CLI가 출력한 안내를 표시한다. 오류 문자열을 해석해 새 요청을 보내거나 CLI를 자동 재시작하지 않는다. 네이티브 CLI 자체의 네트워크 재시도는 별개이며 앱이 끄거나 횟수를 보장하지 않는다.

## 확인한 실행 실패

- 로컬 편집기 서버 시작 실패를 주입했다. 긴 경로용 임시 소켓 디렉터리가 남는 문제를 수정했고, 원래 오류가 전달되며 빈 임시 디렉터리가 제거되는 회귀 검사 1개가 통과했다.
- 설치된 Claude·Codex에 존재하지 않는 옵션을 전달했다. 두 CLI 모두 0이 아닌 코드로 종료했고 오류 옵션이 기록에 남았다. 종료 버퍼와 종료 코드 표시가 유지되며 추가 Enter는 거절됐다. 동시에 실행한 시험 PTY는 살아 있었다.
- 각각 새 세션에서 `--version`이 정상 종료했다. 이는 실패 후 프로세스 생성 경로의 복구 확인이며, 인증이나 모델 응답 성공 증거는 아니다.

재현: `/Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q -L lisp -l tests/cli-startup-errors.el --eval '(ert-run-tests-batch-and-exit "emacs-ai-installed-cli-startup-errors")'`. 실제 설치 CLI를 실행하므로 일반 가짜 응답 회귀 검사에는 포함하지 않는다. 결과 로그: `var/cli-startup-errors-20260909.log`. 시험 기록은 검사 중 확인 후 임시 디렉터리와 함께 정리한다. 모델 프롬프트는 보내지 않았다.

## 사용 중 오류가 발생하면

CLI가 살아 있으면 해당 화면의 오류와 재시도 안내를 먼저 확인한다. 재요청 전 직전 작업이 파일을 변경했는지 확인한다. 프로세스가 종료됐다면 종료 버퍼나 Terminal AI 메뉴의 기록 열기로 오류를 확인하고, 시작 화면의 재개에서 기존 대화를 선택한다. 앱의 원시 기록 열기는 CLI 대화 재개와 다르다. [재개 절차](resume-entry.md).

인증 만료·구독 한도 도달은 아직 검증하지 않았다. 로그아웃, 자격 증명 삭제, 한도 소진으로 시험하지 않는다. 네트워크 실패의 아래 재현은 실제 Wi-Fi 단절이나 모든 전송 경로의 차단을 증명하지 않는다.

## 실제 CLI 연결 실패와 온라인 재개

`tests/cli-offline.py PROVIDER NEW_DIRECTORY`로 기존 PTY 검증 드라이버를 실행했다. 런처는 127.0.0.1의 임의 포트를 bind한 채 listen하지 않아 다른 서비스가 해당 포트를 차지하지 못하게 한다. 자격 증명을 받거나 기록하는 프록시 서버가 아니다. 자식 환경의 대소문자 HTTP_PROXY/HTTPS_PROXY/ALL_PROXY만 그 주소로 지정하고 NO_PROXY는 비웠다. 시스템 네트워크 설정·계정 설정 파일을 수정하지 않았다. CLI가 환경 프록시를 따르지 않는 모든 부가 연결까지 차단했다고 주장하지 않는다.

| 단계 | Claude 2.1.263 | Codex 0.153.4 |
| --- | --- | --- |
| 연결 실패 | Connection refused, 네이티브 재시도 attempt 1/10 | MCP 초기화 실패, 요청 중 Reconnecting 3/5 및 Operation timed out |
| Escape | 요청 중단 후 원래 문장을 입력란에 복원 | Conversation interrupted, 빈 입력란 |
| 정상 종료 | 빈 입력 확인 후 /exit, exit 확인 | /quit, exit 확인 |
| 프록시 없는 새 프로세스에서 같은 ID 재개 | 기존 실패 요청 표시, 새 요청에 NETWORK_RECOVERY_OK | 기존 중단 요청 표시, 새 요청에 NETWORK_RECOVERY_OK |

Claude 중단 후 복원된 입력에 /exit를 붙이는 시험 조작 오류가 있었다. 추가 요청을 즉시 중단한 뒤 Ctrl-U로 입력란을 비우고 화면 확인 후 /exit만 제출했다. 따라서 오프라인 요청 제출은 Claude **2회**, Codex **1회**이고 온라인 복구 요청은 각각 **1회**다. 오프라인 모델 응답은 관찰하지 못했고 온라인 답변은 각각 1개다. 서비스 측 과금·네트워크 요청 수를 측정한 것은 아니다. 앱의 프롬프트 자동 삽입과 네이티브 CLI의 재시도는 구분한다.

복구 시나리오는 같은 프로세스의 네트워크 실시간 복원이 아니라 **명시적 정상 종료 후 ID 재개**다. 두 온라인 CLI·오프라인 CLI 모두 정상 종료했고, 네 개 시험 배치 Emacs 및 두 프록시 포트 예약 런처가 종료 코드 0으로 끝났다. GUI 입력 지연 검사는 아니다.

증거: [Claude 재시도](cli-validation-evidence/offline/claude-retry.txt), [Codex 재시도](cli-validation-evidence/offline/codex-retry.txt), [Claude 복구 응답](cli-validation-evidence/offline/claude-recovered.txt), [Codex 복구 응답](cli-validation-evidence/offline/codex-recovered.txt). 종료 화면은 같은 디렉터리의 `*-exit.txt`, 실행 환경과 원시 기록 위치는 `var/{offline,online}-{claude,codex}-20260909/`에 남겼다. 재현 드라이버는 정상 종료까지 사용자가 제어하는 수동 실제 연결 검사이며 자동 회귀에 포함하지 않는다.
