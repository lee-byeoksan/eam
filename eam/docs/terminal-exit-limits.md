# 터미널 종료와 실행 중 작업의 수명

2026-09-08. **현재 앱은 CLI/Emacs가 종료되었을 때 모든 하위 작업이 종료된다고 보장하지 않는다.** 실제 Claude와 Codex가 각각 시험 명령을 실행 중일 때 CLI 프로세스만 SIGKILL로 종료했으나, 두 명령 모두 계속 실행됐다. 이는 앞선 유휴 CLI 종료 격리 시험에서 다루지 않은 범위다.

## 실제 CLI 결과

| 시험 | Claude 2.1.263 | Codex 0.153.4 |
| --- | --- | --- |
| `python3 -u exit_probe.py` 실행·heartbeat 변화 확인 | 통과 | 통과 |
| 실행 중 CLI PID에 SIGKILL | CLI 상태 signal | CLI 상태 signal |
| 2초 후 heartbeat 계속 변화 | **작업 생존** | **작업 생존** |
| 시험 제어기가 알려진 작업 PID를 따로 종료 | 정리 완료 | 정리 완료 |
| 정리 후 해당 명령 PID 조회 | 없음 | 없음 |

이 시험의 Python 명령은 PID와 heartbeat만 시험 디렉터리에 기록하고 90초 이내 스스로 종료하도록 작성했다. 자연 완료를 잘못 성공으로 기록하지 않도록 강제 종료 직전에 heartbeat 변화를 확인했다. Claude의 해당 셸 명령은 화면에서 확인하고 한 번만 승인했다. Codex는 시험 workspace 정책으로 실행했다. 추가 AI 요청은 제공자별 한 번이며, 최초 검증부터 누계는 각각 8회다.

증거: [Claude](cli-validation-evidence/stream-and-exit/claude-kill-result.json), [Codex](cli-validation-evidence/stream-and-exit/codex-kill-result.json). 이 두 시험은 CLI 강제 종료를 검증했다. 실제 제공자에서 정상 `/exit`·`/quit`, Close, Emacs 강제 종료를 실행 중 작업과 조합한 모든 경우를 시험한 것은 아니다.

## 가짜 프로세스 계층으로 분리 검사

`tests/lifecycle-fixture.py`는 같은 프로세스 그룹의 일반 자식과 `start_new_session=True`로 분리한 자식을 만든다. 각 자식은 heartbeat를 기록하며 45초 후 자동 종료한다. 실제 Ghostel/Emacs PTY에서 관찰한 결과:

| 종료 동작 | 같은 그룹의 자식 | 별도 세션의 자식 |
| --- | --- | --- |
| 앱 Close | 작업 중지 | 작업 생존 |
| CLI만 SIGKILL | 작업 중지 | 작업 생존 |
| Emacs 정상 종료 | 작업 중지 | 작업 생존 |
| Emacs SIGKILL | 작업 중지 | 작업 생존 |

[전체 결과](cli-validation-evidence/stream-and-exit/fixture-lifecycle.json). ‘중지’는 종료 후 heartbeat가 더 이상 변하지 않는다는 관찰이다. 모든 프로세스의 reap까지 보증하는 표현이 아니다. 각 시험 종료 뒤 명령 문자열과 시험 디렉터리로 신원을 확인한 PID만 따로 정리했다.

Ghostel을 제외한 Python `pty.fork` 대조군에서는 master를 닫은 뒤 일반 자식과 분리된 자식 **둘 다** 계속 동작했다. HUP/INT/TERM을 기본 처리로 초기화한 재검사도 같았다. [기본 대조](cli-validation-evidence/stream-and-exit/plain-pty.json), [신호 초기화 대조](cli-validation-evidence/stream-and-exit/plain-pty-default-signals.json). Emacs의 프로세스 그룹 종료와 단순 master FD 닫기가 같은 동작은 아니다. 이 대조로 확인한 공통점은 분리된 자식의 생존이며, 두 터미널의 모든 종료 동작이 같다고 결론 내리지 않는다.

## 현재 사용 시 의미

후속 [정상 종료 검사](cli-normal-exit.md)에서는 Claude `/exit`의 `Exit and stop tasks` 선택과 Codex `/quit`가 각각 시험 작업을 정리했다. 강제 종료 결과와 구분한다.

- 실행 중 명령을 멈출 때는 먼저 CLI 자신의 중단 동작을 사용한다. 앞선 실제 시험에서 Claude Escape는 시험 명령을 종료했고, Codex Escape는 백그라운드 명령을 남겨 `/stop`이 추가로 필요했다. 이 역시 모든 종류의 명령에 대한 보장은 아니다.
- **Close는 전체 프로세스 트리 정리 명령이 아니다.** CLI 강제 종료나 Emacs 충돌 뒤에는 별도 백그라운드 작업이 남을 수 있다.
- 전체 기록은 PTY가 받은 출력의 기록이다. CLI가 다른 파일로 돌린 백그라운드 출력까지 자동 수집하는 것은 아니다.

현재 제품에 임의의 전체 자식 강제 종료를 추가하지 않았다. 세션에서 띄운 개발 서버나 사용자가 의도적으로 분리한 작업을 종료할 수 있고, 공유 CLI 데몬과 특정 작업의 소유 관계도 구분해야 하기 때문이다. CLI가 이미 죽은 뒤에는 단순 부모 PID 조회만으로 원래 소유 관계를 복원할 수 없다.

후속 [외부 감시자 실험](lifecycle-supervisor-experiment.md)에서는 미리 관찰한 분리 작업을 Close·CLI/Emacs 충돌 뒤 정리했지만, 관찰 사이에 빠르게 분리된 작업이 남는 반례도 재현했다. 따라서 기본 앱에 연결하지 않았다. ‘세션과 함께 종료할 작업’과 ‘유지할 작업’의 구분, 충돌에도 유효한 소유권 확보는 여전히 미해결이다. **종료 후 작업 잔존은 현재 알려진 안정성 제한으로 유지한다.**

## 재현

```sh
python3 tests/run-lifecycle-probe.py var/새로운-lifecycle-시험경로
```

기존 결과 폴더는 재사용하지 않도록 거절한다. 소켓·PTY·프로세스 조회가 허용된 환경이 필요하다. 이 명령은 네 개의 가짜 계층 시험만 실행하고 AI를 호출하지 않는다. 실제 CLI 강제 종료 검사는 별도의 `tests/kill-live-probe.py`와 `tests/cli-validation-driver.el`을 사용하며, 살아 있는 자체 시험 명령과 CLI 신원을 먼저 확인한다.
