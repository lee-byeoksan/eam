# 지속 세션

모든 세션은 Emacs와 독립된 Rust PTY 데몬에서 실행한다. `eam-new`로 시작하고 `eam-detach`로 화면을 닫으며, `eam-attach`로 살아 있는 CLI에 다시 연결한다. CLI 정상 종료 또는 `eam-quit`은 세션을 종료한다. 종료한 대화는 `eam-resume`으로 CLI 원본 기록에서 재개한다.

현재 구현·제한은 [PTY 백엔드](pty-backend.md), [native runtime과 검증](native-runtime.md), [사용 매뉴얼](user-guide.md)을 따른다. 백엔드 선택 설정은 없다. 이전 백엔드의 공통 검증 기록은 저장소의 evidence 아래 보존하며 제품 tar에는 포함하지 않는다. 개인 세션 기록과 실행 프로세스는 이 정리에서 변경하지 않았다.

5MiB 시작 출력 재생은 완전한 화면 snapshot이 아니며 CLI 자체 기록은 history로 조회한다. 실제 GUI 한글 조합과 스크롤 체감은 가짜 CLI 배치 검증과 구분한다.
