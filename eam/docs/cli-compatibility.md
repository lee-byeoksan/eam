# CLI 기능 호환 범위

2026-09-09 G04. Emacs 30.1 / Ghostel v0.53.0 / Claude Code 2.1.263 / Codex 0.153.4에서 얻은 기존 증거를 대조했다. 'CLI 전체 지원' 대신 아래 검증 범위를 사용한다. PTY 성공을 물리 IME 성공으로 확대하지 않는다.

| 기능 | Claude | Codex | 증거 |
| --- | --- | --- | --- |
| 읽기·파일 수정·테스트 | 작은 greet 작업과 재개 후 4개 테스트 | 같은 시나리오 확인 | [코딩 검사](cli-validation.md) |
| 승인·거절 후 복구 | 쓰기 No, 파일 부재, 이후 메뉴 응답 | read-only 시도 후 승인 거절·메뉴 응답 | [후속 검사](cli-validation.md#세션-왕복과-codex-거절-검증--2026-09-08-후속) |
| 모델 변경 | Fable→Sonnet→Fable, 세션 한정 s 선택 및 응답 | 임시 Luna 시작→기존 Astra/xhigh 메뉴 선택 및 응답 | [모델 검사](cli-model-and-soak.md) |
| goal | 유한 목표 설정→파일 생성→완료, 별도 활성 목표 중단→clear 확인 | 이 프로젝트의 검증 증거 없음, 지원 여부 단정하지 않음 | [실제 goal 검증](claude-goal-validation.md) |
| 외부 편집기 | 한글·여러 줄 왕복, GUI와 PTY | 한글·여러 줄 왕복, GUI와 PTY | [경로/왕복](korean-path-validation.md), [GUI](cli-validation.md) |
| 보조 초안 | 전송과 Enter 분리, 64 KiB 상한 | 같은 앱 경로, 실제 3세션 입력 격리 | [다중 Codex](codex-project-sessions.md), terminal-test.el |
| 대화 재개 | ID, 목록 선택 후 후속 응답 | ID, 목록 선택 후 후속 응답 | [재개](resume-entry.md) |
| 작업 중단 | 시험 셸 Escape 후 종료 | 직접 셸 Escape 종료, 도구 셸 Escape 후 생존·/stop 종료 | [중단](cli-validation.md), [정상 종료](cli-normal-exit.md) |
| 작업 목록·정상 종료 | /tasks와 /exit의 stop 선택 확인 | /ps와 /quit 시험 작업 정리 확인 | [정상 종료](cli-normal-exit.md) |
| 강제 종료 | 실행 중 작업 잔존 반례 | 실행 중 작업 잔존 반례 | [제한](terminal-exit-limits.md) |
| 직접 셸 ! | 실행 확인, 완료 후 모델 응답도 관찰 | 직접 실행 확인 | [한글 경로](korean-path-validation.md) |
| 첨부·이미지·파일 칩 | 미검증 | 미검증 | 텍스트 왕복의 범위 밖 |

모델 메뉴는 영구 기본값을 바꿀 수 있다. Claude 검사는 s로 세션 한정 변경했고 Codex는 임시 시작 모델에서 기존 기본값으로 돌아왔다. 전후 설정 파일 내용 불변의 [측정 결과](cli-validation-evidence/model-and-soak/model-checks.json)가 있으나 모든 메뉴 변경의 비영구성을 뜻하지 않는다. 재개 명령은 권한/모델 모드를 강제하지 않는다.

## 남은 기능 검사

- Claude 목표 설정·수행·해제는 작은 파일 조건으로 실제 검증했다. 여러 턴 장기 수행·활성 목표 재개는 미검증이며, Codex에 같은 명령이 있다고 추정하지 않는다.
- 비텍스트 첨부: 현재 텍스트 중심 지원 범위에서는 미지원 보증 항목으로 명시한다. 추가 요구가 생기면 별도 시나리오가 필요하다.
- GUI 합성 입력·IME·모드 전환은 G01, 인증/한도/연결 오류는 G09, 작업 잔존은 G05에서 추적한다. 이미 증명된 파일 수정·승인·모델 왕복은 새 변경이 없으면 반복하지 않는다.

이번 정리는 새 AI 호출 없이 원시 화면·설정 체크 JSON과 실험 기록을 대조한 것이다. 앱은 CLI 명령을 별도 파싱·대체하지 않으며, 키와 텍스트 전달을 제공한다. 따라서 CLI의 버전 변경 뒤에는 관련 명령 검증을 갱신해야 한다.
