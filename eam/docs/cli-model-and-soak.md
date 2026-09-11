# 모델 변경과 다중 CLI 유휴 검사

2026-09-08. Emacs for Mac OS X 30.1, Ghostel v0.53.0, Claude Code 2.1.263, Codex 0.153.4. 실제 CLI를 Ghostel PTY에서 실행한 **배치 검사**다. GUI의 IME·프레임 지연 검사와 구분한다. 제품 기능 변경 없이 검증 코드만 추가·확장했다.

## 모델 변경·응답·원복

| 제공자 | 실제 시험 경로 | 결과 |
| --- | --- | --- |
| Claude | 기본 Fable → `/model`에서 Sonnet 선택 후 `s` → 짧은 응답 → Fable 선택 후 `s`와 확인창 → 후속 응답 | 두 응답 모두 `MODEL_SWITCH_OK`. 설정 파일 내용 불변 |
| Codex | `--model gpt-5.6-luna`로 임시 시작 → 짧은 응답 → `/model`에서 기존 기본값 gpt-6-astra / Extra high 선택 → 후속 응답 | 두 응답 모두 `MODEL_SWITCH_OK`. 설정 파일 내용 불변 |

Claude의 `/model`은 Enter와 `s`의 저장 범위가 다르다. Enter는 개인 기본값을 저장하고 `s`는 현재 세션에만 적용한다. [공식 모델 설정](https://code.claude.com/docs/en/model-config). 실제 선택창에서도 같은 설명을 확인했다. 응답 후 Fable로 원복할 때 캐시 재사용 차이를 설명하는 확인창이 나타나, 화면의 원복 대상을 확인하고 확정했다.

Codex의 `/model` 사용 흐름은 [공식 명령 설명](https://learn.chatgpt.com/docs/developer-commands?surface=cli)을 확인했다. 저장 범위는 설치 버전과 같은 `rust-v0.153.4`의 공식 소스 `chatwidget/model_popups.rs`, `app/event_dispatch.rs`에서 일반 선택이 `PersistModelSelection`을 통해 설정 저장을 요청하는 것을 추가 확인했다. 소스 사본은 `var/provider-reference/codex-*.rs`에 있다.

Codex는 개인 기본값을 다른 값으로 바꾸었다가 복원하는 대신, 시작 옵션으로만 임시 모델을 적용하고 메뉴에서 원래 기본값을 선택했다. 따라서 **Luna 시작 → 네이티브 메뉴로 Astra 전환**의 검증이며, 기본값 저장 없이 임의 모델 사이를 메뉴로 왕복할 수 있다는 뜻은 아니다. 기존 설정의 Astra/xhigh를 읽어서 동일하게 선택했다. CLI는 설정 저장 경로를 실행했지만 전후 파일 내용은 같았다.

두 설정 파일 `~/.claude/settings.json`, `~/.codex/config.toml`의 SHA256을 전후 비교하여 내용 불변을 확인했다. 설정 파일 내용이나 인증 토큰을 검증 보고서에 복사하지 않았다. [확인 결과](cli-validation-evidence/model-and-soak/model-checks.json).

응답 증거: [Claude 변경 후](cli-validation-evidence/model-and-soak/claude-changed-response.txt), [Claude 원복 후](cli-validation-evidence/model-and-soak/claude-restored-response.txt), [Codex 임시 시작](cli-validation-evidence/model-and-soak/codex-temporary-response.txt), [Codex 메뉴 전환 후](cli-validation-evidence/model-and-soak/codex-restored-response.txt). 모델 식별 근거는 CLI의 모델 표시·전환 확인문이다. 서버 측 모델 라우팅을 별도 추적한 것은 아니다.

이번 모델 시험의 사용자 요청은 제공자별 짧은 프롬프트 2회다. 앞선 검증을 포함하면 각각 7회이며, 제공자 내부 모델 호출 횟수와 같지 않다.

## 같은 Emacs에서 실제 CLI 4개

`tests/cli-soak.el`로 같은 배치 Emacs 안에 Claude 2개와 Codex 2개를 실행했다. 초기화 후 약 30초 간격으로 11회 측정했다. 시작부터 마지막 표본까지 337.2초(약 5분 37초), 첫 표본부터 마지막 표본까지 약 320초다. 각 회차에 모든 CLI의 `/model` 표시를 확인하고 Escape로 닫았다. **44회 모두 메뉴 표시 성공**했고 출력 undo도 계속 비활성 상태였다. 이 시험에서는 모델 질문을 제출하지 않았다.

| 프로세스 | RSS MiB: 첫 표본 → 마지막 | 메뉴 시간 중앙값 / 최대 ms | 최대 출력 버퍼 문자 |
| --- | ---: | ---: | ---: |
| claude 1 | 335.9 → 355.7 | 295.6 / 317.6 | 649 |
| claude 2 | 335.3 → 356.3 | 270.2 / 291.9 | 649 |
| codex 1 | 264.8 → 266.4 | 246.5 / 251.5 | 623 |
| codex 2 | 245.4 → 223.1 | 246.7 / 256.6 | 669 |

호스트 Emacs RSS는 53.8 → 56.6 MiB였다. RSS는 프로세스별 `ps` 값이며, 위 CLI 값에 별도 자식 프로세스·MCP·공유 데몬 메모리를 합산하지 않았다. 값들을 단순 합산하여 시스템 전체 고유 메모리라고 해석하면 안 된다.

메뉴 시간은 paste부터 메뉴 텍스트 확인까지다. paste 뒤 **의도적인 200ms 대기**, PTY 처리, VT 파싱, 강제 배치 redraw와 최대 50ms 간격의 관찰이 포함된다. 메뉴 Escape 후 대기는 제외한다. GUI 프레임·키보드 지연이나 AI 응답 시간 측정이 아니다. 측정 중 별도 Emacs에서 모델 검증도 수행했으므로 시스템 부하를 완전히 통제한 벤치마크는 아니다.

[전체 시계열](cli-validation-evidence/model-and-soak/samples.json). 코드가 저장하는 RSS·화면 길이는 같은 함수 호출의 원자적 표본이 아니라 순서대로 읽은 값이다. 큰 출력 없이 유휴·메뉴 반복만 수행했다. 일부 RSS 증가·감소가 있지만 이 짧은 표본으로 메모리 누수 유무나 장시간 안정성을 결론 내리지 않는다. 기존 합성 대량 출력 벤치마크와 동일 조건도 아니다.

## 비정상 종료 격리

측정 종료 후 **유휴 Claude 한 개**에 SIGKILL을 보냈다. 상태는 signal이 됐고 마지막 출력 버퍼와 36,828바이트 ANSI 기록은 남았다. 다른 Claude 1개·Codex 2개는 모두 다시 `/model` 메뉴에 응답했다. [강제 종료 결과](cli-validation-evidence/model-and-soak/abrupt-exit.json).

시험 종료 시 네 세션을 Close로 정리하고 Emacs 서버를 종료했다. 알려진 호스트 Emacs·네 CLI PID가 모두 사라진 것도 확인했다. 실행 중인 도구의 자식·손자 프로세스 정리나 모든 비정상 종료 유형을 검증한 것은 아니다. 이 시험의 SIGKILL 대상은 작업을 실행하지 않는 유휴 CLI였다.

## 재현과 남은 범위

```sh
EMACS_AI_SOAK_DIR="$PWD/var/cli-validation/새로운-soak-경로" \
  /Applications/Emacs.app/Contents/MacOS/Emacs --batch -Q -L lisp \
  -l tests/cli-soak.el
```

현재 soak는 기존 `var/cli-validation/20260908/{claude,codex}/workspace` 시험 폴더를 사용한다. AI 질문은 보내지 않지만 실제 CLI 시작에 따른 인증·네트워크·기존 MCP 시작은 발생할 수 있다. 일반 `scripts/test.sh`에는 포함하지 않았다. 로컬 Unix 소켓과 프로세스 RSS 조회가 허용되는 실행 환경이 필요하다. 실패 시 추가 입력을 중단하고 시험 세션들을 정리한다.

모델 검증 driver는 `EMACS_AI_VALIDATION_MODEL`을 지정하면 공식 `--model` 시작 옵션을 전달한다. 이번 변경 코드 두 파일은 실제 실행을 마쳤고 Lisp 괄호 검사도 통과했다. 기존 제품 코드와 회귀 테스트는 수정하지 않았다.

남은 핵심 범위:

1. 수십 분 이상 실제 출력 부하, 동시에 입력·스크롤·리사이즈하는 GUI 반응, 장시간 RSS 추세.
2. 도구 실행 중 CLI/Emacs의 비정상 종료와 하위 작업 잔존 여부.
3. 물리 키보드 한글 조합과 비텍스트 첨부. 합성 입력 성공으로 대체하지 않음.

작은 실제 코딩·승인/거절·중단·세션 재개·모델 전환과 유휴 다중 세션은 검증했지만, 일상 사용 안정화가 완료된 상태는 아니다.
