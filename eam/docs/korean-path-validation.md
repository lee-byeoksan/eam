# 한글·공백 경로와 입력 편집 왕복

2026-09-08. 실제 Claude Code 2.1.263·Codex 0.153.4를 별도 배치 Emacs/Ghostel에서 실행했다. cwd는 제공자별 `var/cli-validation/20260908-korean-path/{claude,codex}/한글 프로젝트`다.

두 CLI 모두 다음 검사를 통과했다.

1. `!` 직접 셸로 Python 명령을 실행해 cwd의 `결과 파일.txt`에 `한글 경로 확인🙂`를 기록했다. 파일 경로와 내용을 대조했다.
2. CLI 입력란에 미전송 한글·이모지 두 줄을 붙여넣고 Ctrl-G로 Emacs 외부 편집기를 열었다. 열린 파일의 원문이 입력과 정확히 같았다.
3. 다른 한글 두 줄로 편집·저장하고 CLI 입력란으로 반환했다. Ctrl-G로 다시 열어 수정한 두 줄과 정확히 같은지 확인했다.
4. 미전송 입력을 비운 뒤 native `/exit` 또는 `/quit`로 정상 종료했다. CLI exit와 archive error 없음 확인 후 시험용 배치 Emacs를 종료했다.

[검사 결과](cli-validation-evidence/korean-path/checks.json), [Claude 직접 셸](cli-validation-evidence/korean-path/claude-shell-screen.txt), [Codex 직접 셸](cli-validation-evidence/korean-path/codex-shell-screen.txt), [Claude 재개방 원문](cli-validation-evidence/korean-path/claude-editor-reopened.txt), [Codex 재개방 원문](cli-validation-evidence/korean-path/codex-editor-reopened.txt).

시험 driver에 현재 서버 편집 중인 버퍼가 정확히 하나인지 확인한 뒤 읽기·편집·반환하는 검증 함수를 추가했다. `cli-validation-control.py`의 `editor-read`와 `editor-write`로 재현할 수 있다. 개인 GUI 인스턴스에는 연결하지 않는다.

**AI 호출 기록 정정:** 명시적으로 모델에 작업을 요청한 프롬프트는 두 제공자 모두 0회였지만, Claude는 직접 셸 명령 완료 뒤 모델 설명 응답 1회를 생성했다. 최초 기록의 ‘AI 요청 0회’를 실제 호출 없음으로 읽어서는 안 된다. [네이티브 응답 감사](cli-validation-evidence/korean-path/claude-native-response-audit.json)에서 `claude-fable-5-1` 응답 ID와 출력 128토큰을 확인했다. `!`가 셸 명령을 직접 실행한다는 사실만으로 후속 모델 호출도 없다고 가정한 것이 잘못이었다. 호출이 없어야 하는 향후 검증에는 이 Claude 경로를 사용하지 않는다. 앱이 시스템 프롬프트나 컨텍스트를 삽입한 것은 아니다.

편집한 한글 초안 자체는 모델에 제출하지 않았다. GUI·물리 키보드 IME 조합 검증이나 매우 긴 Unix 소켓 경로의 한계 검증은 아니다. 앱 제품 코드의 경로 처리 수정 없이 통과했다.

## 새 Emacs에서 재개

정상 종료 뒤 같은 한글 프로젝트에서 새 배치 Emacs를 실행했다. Claude는 `/resume` 목록의 직전 대화를 선택했고, Codex는 종료 시 표시한 native ID로 재개했다. 둘 다 기존 셸 명령과 `PATH_OK`를 복원했다. [Claude 목록](cli-validation-evidence/korean-path/claude-resume-picker.txt), [Claude 복원](cli-validation-evidence/korean-path/claude-resumed.txt).

Codex의 footer는 재개 뒤 `default`를 표시했지만 `/status`는 `reasoning xhigh`를 표시했다. [동일 화면 발췌](cli-validation-evidence/korean-path/codex-resumed-status-excerpt.txt). footer만 보고 실제 설정이 바뀌었다고 결론 내리지 않았고 설정을 변경하지 않았다. 후속 모델 요청의 실제 reasoning 적용까지 검사한 결과는 아니다. 재개 검사를 끝낸 두 시험 CLI와 Emacs도 정상 종료했다.
