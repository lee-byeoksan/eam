# Claude 네이티브 goal 검증

2026-09-09, Claude Code 2.1.263 / Emacs 30.1 / Ghostel v0.53.0. 실제 구독 CLI의 배치 PTY에서 실행했다. 앱에 별도 goal 구현이나 평가 요청을 추가하지 않았다.

| 단계 | 확인한 결과 |
| --- | --- |
| `/goal` | No goal set, 설정 문법 표시 |
| `/goal <유한 조건>` | 별도 프롬프트 없이 실행 시작, /goal active 표시 |
| 목표 수행 | goal_result.txt 생성 diff의 해당 작업만 승인. CLI가 바이트 덤프와 크기를 확인 |
| 완료 | Goal achieved, 35s·1 turn·591 tokens 표시. 독립 파일 읽기에서도 정확히 `GOAL_OK\n` 8바이트 |
| 완료 상태 조회 | `/goal`에 완료한 조건과 수치 표시 |
| 두 번째 목표 중단 | cancel_result.txt 생성 승인에서 Escape. 파일은 없지만 목표는 active로 유지 |
| `/goal clear` | Goal cleared와 대상 조건 표시, active 표시 제거, 취소 파일 부재 유지 |
| 종료 | `/exit`로 CLI exit 확인, 시험 Emacs 종료 코드 0 |

명시적으로 설정한 목표는 2개(성공·취소)다. 첫 목표는 파일 하나로 끝나는 조건이었고, 두 번째는 파일 생성 전에 중단했다. 장시간 자율 수행·여러 턴 반복·활성 목표 재개·모든 실패 유형을 검증한 것은 아니다. 표시된 토큰 수는 CLI 보고값이며 실제 과금이나 모든 내부 호출 수의 계측이 아니다.

[공식 goal 문서](https://code.claude.com/docs/en/goal)를 열어 확인했다. `/goal`은 별도 모델 평가를 사용하고, 활성 목표 해제는 `/goal clear`다. 따라서 **앱의 추가 평가 호출 없음**과 **네이티브 goal의 평가 호출 없음**을 혼동하지 않는다. 이번에는 실제 파일을 독립적으로 읽어 완료를 확인했다. 네이티브 평가기의 완료 선언만으로 결과를 확정하지 않았다.

증거: [초기 상태](cli-validation-evidence/goal/before.txt), [생성 승인](cli-validation-evidence/goal/create-approval.txt), [완료](cli-validation-evidence/goal/achieved.txt), [완료 상태](cli-validation-evidence/goal/achieved-status.txt), [취소 전](cli-validation-evidence/goal/cancel-before.txt), [해제](cli-validation-evidence/goal/cleared.txt). 원시 PTY 위치는 `var/goal-check-20260909/state.json`, 명령 원문·실제 결과 파일은 같은 디렉터리에 보관한다. 실제 GUI와 물리 IME 검사는 별도다.
