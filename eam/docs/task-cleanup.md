# 종료 후 남은 작업 처리

2026-09-09 G05. 기본 방침은 CLI의 정상 중단·종료를 먼저 사용하고, 충돌 후에는 **소유가 확인된 작업만 명시적으로 종료**하는 것이다. 전체 자식 자동 종료는 적용하지 않는다. 기존 감시자는 관찰 전에 분리된 프로세스를 놓쳤으며, 다른 세션이나 의도적으로 유지한 개발 서버를 자동 종료할 위험이 있다.

## 정상 종료

- Claude: `/tasks`에서 작업 확인 → `/exit` → 작업까지 중지하려면 `Exit and stop tasks`.
- Codex: `/ps`에서 관리 작업 확인 → 필요하면 `/stop` → `/quit`.
- 이후 앱 Close로 출력·초안 버퍼를 닫는다. 앱 Close 자체는 모든 하위 작업을 정리하는 명령이 아니다.

이 순서로 실제 두 CLI의 시험 작업 정리를 확인한 [기존 결과](cli-normal-exit.md)를 재사용한다. 실행 중 강제 종료 시 두 CLI 모두 작업이 남았던 [반례](terminal-exit-limits.md)도 유효하다.

## 충돌 후 알려진 작업 하나 정리

`scripts/task-process.py`는 작업의 실행 기록·PID 파일 등으로 **사용자가 소유를 확인한 PID**를 받아 신원을 조회하고, 조회한 신원과 현재 신원이 같을 때 SIGTERM을 보낸다. 작업 이름으로 전체 프로세스를 검색하거나 다른 PID를 자동 추정하지 않는다.

```sh
# PID를 실제로 확인한 작업 번호로 바꾼다.
python3 scripts/task-process.py inspect PID > var/task-to-stop.json
cat var/task-to-stop.json
# PID, uid, sec/usec(프로세스 시작 시각)를 확인한 뒤 실행한다.
python3 scripts/task-process.py stop var/task-to-stop.json
```

receipt는 종료 권한이나 AI 세션 소유 증명이 아니다. CLI가 죽은 뒤 부모 PID가 바뀌어도 PID·UID·시작 시각으로 같은 프로세스인지 확인한다. 신원이 다르거나 프로세스 조회가 실패하면 신호를 보내지 않는다. 현재 사용자 소유의 별도 작업만 대상으로 허용한다. 환경변수·전체 명령 인자는 수집하지 않는다.

성공 출력은 **신호 전송**을 뜻한다. 작업이 SIGTERM을 무시할 수 있으므로 종료됐는지는 작업의 상태·heartbeat 또는 PID 조회로 별도 확인한다. 자동 SIGKILL 승격이나 전체 트리 정리는 하지 않는다. 자식이 또 남아도 자동으로 쫓아가 죽이지 않는다. OS의 신원 확인과 신호 전송 사이 PID 재사용 경쟁은 완전히 없애지 못한다.

## 검증과 남은 제한

`python3 tests/task-process-test.py`는 실제 macOS에서 자기 시험 프로세스 두 개를 각각 별도 세션으로 분리해 실행했다. 잘못된 시작 시각 receipt는 거절되고 대상이 살아 있는 것, 올바른 receipt로 지정한 작업만 SIGTERM 종료(-15)하는 것, 다른 작업은 계속 실행되는 것을 확인했다. 시험 후 남은 자기 프로세스만 정리했다. AI 호출은 없다.

libproc 신원 조회는 기존 `experiments/lifecycle/watchdog.py`의 검증된 읽기 함수만 재사용한다. 해당 모듈을 import해도 감시자나 자동 종료 루프는 실행되지 않는다. 전체 트리 소유권 확보는 여전히 미해결이며, 이 도구가 기존 감시자 반례를 해결한 것은 아니다. 모르는 PID를 대상으로 추측해서 사용하지 않는다.
