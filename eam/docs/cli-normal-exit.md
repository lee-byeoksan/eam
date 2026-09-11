# 네이티브 작업 확인과 정상 종료 검증

2026-09-08. Emacs 30.1/Ghostel, Claude Code 2.1.263, Codex 0.153.4의 **실제 PTY 배치 검사**다. 이번 정상 종료에서 시험 작업은 정리됐다. 강제 종료·Emacs 충돌이나 임의로 분리된 모든 자식에 대한 보장은 아니다.

## 관찰 결과

| 실행 경로와 조작 | 작업 관찰 | 결과 |
| --- | --- | --- |
| Claude `!python3 -u probe.py native` → Ctrl+B | heartbeat 계속 변화, `/tasks`에 running 표시 | 직접 셸 작업의 백그라운드 전환·조회 통과 |
| Claude `/exit` | 실행 중 작업과 `Exit and stop tasks` / `Move to background and exit` / `Stay` 선택 표시 | 종료와 유지 선택을 CLI 자체가 제공 |
| Claude `Exit and stop tasks` 선택 | CLI exit, heartbeat 정지, probe PID 조회 결과 없음 | 이번 일반 백그라운드 셸 정리 통과 |
| Codex `!python3 -u probe.py native` → Escape | heartbeat 정지, `/ps`에 작업 없음, probe PID 조회 결과 없음 | 직접 셸 작업 중단 통과 |
| Codex AI 도구로 `python3 -u probe.py managed`, yield 1000 ms 실행 | 응답 완료 뒤에도 heartbeat 변화, `/ps`에 명령 표시 | 도구 경로의 백그라운드 작업 조회 통과 |
| Codex `/quit` | CLI exit, heartbeat 정지, probe PID 조회 결과 없음 | 이번 background terminal 정리 통과 |

Claude `/exit`를 입력한 직후에는 선택창이 열린 채 CLI와 작업이 계속 실행됐다. 이를 종료 실패로 세지 않고, 선택을 확정한 뒤 상태와 heartbeat를 다시 측정했다. Codex의 직접 셸과 AI 도구 실행은 중단 동작을 같은 것으로 일반화하지 않는다. 앞선 AI 도구 작업은 Escape 뒤 남아 `/stop`이 필요했고, 이번 직접 셸 작업은 Escape만으로 중지됐다.

증거: [PID 부재·요청 수](cli-validation-evidence/normal-exit/checks.json), [Claude 작업 화면](cli-validation-evidence/normal-exit/claude-tasks-running.txt), [Claude 종료 선택](cli-validation-evidence/normal-exit/claude-exit-choice.txt), [Claude 결과](cli-validation-evidence/normal-exit/claude-normal-exit-observation.json), [Codex 직접 셸 중단](cli-validation-evidence/normal-exit/codex-shell-after-escape.txt), [Codex 작업 목록](cli-validation-evidence/normal-exit/codex-managed-tasks.txt), [Codex 결과](cli-validation-evidence/normal-exit/codex-normal-exit-observation.json).

## 실제 사용 순서

- **Claude:** 실행 중 응답은 Escape로 중단하고, 백그라운드 작업은 `/tasks`에서 확인한다. 종료하려면 `/exit`를 사용하고, 작업까지 종료하려는 경우 `Exit and stop tasks`를 선택한다. `Move to background and exit`는 유지 목적의 다른 선택이다. 이번에는 그 유지 선택 자체를 실행하지 않았다.
- **Codex:** Escape 뒤에도 작업이 남을 수 있으므로 `/ps`로 확인한다. 현재 세션의 background terminal을 중단하려면 `/stop`, CLI를 정상 종료하려면 `/quit`를 사용한다. `/ps`가 빈 목록이라고 모든 OS 자식이나 외부 데몬이 없다는 뜻은 아니다.
- CLI가 종료되면 출력 버퍼와 원시 기록은 남는다. 그 뒤 앱의 `Close experiment`로 버퍼를 닫을 수 있다. **실행 중 CLI에서 Close를 누르거나 Emacs를 바로 종료하는 동작은 위 네이티브 종료와 같지 않다.**

공식 근거: Claude의 [직접 셸·백그라운드 작업](https://code.claude.com/docs/en/interactive-mode), Codex의 [작업 목록·중단 명령](https://learn.chatgpt.com/docs/developer-commands?surface=cli). Codex 공식 문서의 `/ps`는 `unified_exec`가 관리하는 background terminal 범위다. 네이티브 CLI 버전·실행 모드에 따라 메뉴와 동작이 달라질 수 있다.

## 재현과 범위

`tests/cli-validation-driver.el`로 제공자별 새 배치 Emacs를 시작하고 `tests/cli-validation-control.py`로 입력했다. cwd는 `var/cli-validation/20260908-normal-exit/{claude,codex}/workspace`였다. [시험 명령 원본](cli-validation-evidence/normal-exit/probe.py)은 PID·heartbeat만 기록하고 최대 180초 후 스스로 종료한다. 종료 동작 직전 heartbeat 변화를 확인해 자연 완료와 구분했다.

Claude와 Codex의 `!` 셸 실행에는 모델 작업 프롬프트를 명시적으로 보내지 않았다. Codex AI 도구 경로 검증에는 `exec_command`로 해당 명령 하나를 실행하고 기다리지 말라는 요청 1회만 제출했다. 이번 명시적 요청은 Claude 0회·Codex 1회이며 누계는 각각 8회·9회다. CLI 내부 호출 수나 시작 네트워크 횟수는 아니다. 특히 후속 [한글 경로 검사](korean-path-validation.md)에서는 Claude가 `!` 명령 완료 뒤 모델 응답을 생성했다. 직접 셸을 사용했다는 이유만으로 실제 모델 호출 0회를 보장하지 않는다.

시험용 Emacs 둘 다 마지막에 종료했다. 개인 Emacs 설정이나 사용자 CLI 기본 설정을 수정하지 않았고 새 GUI를 실행하지 않았다. 이번에 Claude의 유지 선택, 의도적인 double-fork, 도구의 모든 자식 계층, 실제 GUI/물리 키보드를 검증한 것은 아니다. [강제 종료 제한](terminal-exit-limits.md)과 [감시자 반례](lifecycle-supervisor-experiment.md)는 계속 유효하다.
