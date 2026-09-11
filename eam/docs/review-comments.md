# F03 diff 리뷰 의견

2026-09-10 구현·설치 검증. `emacs-ai-review-selection` / `C-c a v`.
0.4.0 패키지에 포함됐다.

선택한 diff만 복사하고 unified diff hunk에서 변경 전·후 줄 범위를 계산한다.
삭제된 줄은 새 파일 줄 번호를 none으로 표시한다. 한글·공백 파일명과 Git의
따옴표/8진수 UTF-8 파일명을 처리한다. 여러 hunk와 여러 파일의 위치를 나눠 기록한다.
선택 내용에 코드 펜스가 있으면 더 긴 펜스를 사용한다.

의견과 대상 세션 선택이 끝나기 전에는 초안을 만들거나 바꾸지 않는다.
초안은 기존 내용 뒤에 추가하고 undo 경계를 유지한다. CLI 시작·전송·AI 호출은
없으며, 사용자가 초안에서 수정한 후 기존 전송 명령을 실행한다.
선택 길이는 기본 12,000자이며 초과하면 실제 길이와 제한을 표시하고 중단한다.

## 검증

`tests/review-test.el` 5개 통과: 변경 후 줄 위치, 여러 hunk·파일, 한글 경로,
삭제 줄, 헤더처럼 보이는 추가 줄, 선택 한도, 펜스, 취소 시 초안 미생성,
전송 함수 미호출, 초안 undo. 앱 회귀 15개 통과.

## 남은 확인과 한계

일반 unified diff 형식이 대상이며 combined diff·바이너리 diff는 지원하지 않는다.
위치는 diff 생성 시점 기준으로 현재 파일과 다를 수 있다. Magit에서는 `magit-diff-file-header`로 원래 diff 헤더를 읽는다.
실제 Magit 4.4.0의 배치 버퍼에서 이름 변경과 접힌 구간을 검증했다.
다른 버전과 실제 GUI 표시 차이까지 검증했다고 간주하지 않는다.
일반 Emacs GUI의 영역 선택·의견 입력·초안 편집과 설치된 패키지 검증도 남아 있다.

## 실제 Magit 버퍼 검증

`bash scripts/test-review.sh --magit`: 일반 diff 5개와 실제 Magit 테스트 1개 통과.
[Magit 실행 로그](evidence/review/magit.log),
[원본 의존성 리비전](evidence/review/magit-dependencies.json).
Magit 4.4.0과 필요한 원본 의존성을 프로젝트 `var/deps`에만 내려받았으며
개인 Emacs에 설치하거나 의존성 소스를 수정하지 않았다.

임시 Git 저장소에서 한글·공백 경로의 파일을 커밋하고 이름을 바꾼 뒤 한 줄을
수정·스테이징했다. Magit 자체가 만든 diff 버퍼에서 선택한 줄의 이전 경로,
새 경로, 변경 후 21번 줄, 정확한 선택 본문을 확인했다. 같은 hunk를 접으면
캡처를 거절하고, 다시 펼치면 캡처되는 것도 확인했다.

최초 테스트는 이전 경로를 새 경로로 표시하는 제품 오류를 재현했다.
`magit-file-at-point` 하나로 두 경로를 채우던 코드를 원본 diff 헤더 기반으로
수정했다. 접힌 텍스트가 선택 영역에 포함되면 펼친 뒤 재선택하도록 안내하므로,
보이지 않는 내용을 초안에 자동 포함하지 않는다.

## 0.4.0 설치본 검증

[산출물](evidence/review/package-0.4.0.json),
[PTY·리뷰 로그](evidence/review/package-pty-0.4.0.log),
[실제 Magit 로그](evidence/review/package-magit-0.4.0.log).
설치본에서 앱 15개, 리뷰 5개, 알림 4개, PTY 6개, Magit 1개로
서로 다른 테스트 31개가 통과했다. Magit 및 리뷰 함수가 소스 작업 폴더가 아닌
패키지 설치 경로에서 로드됐는지 확인했다. 실제 AI는 호출하지 않았다.

사용: `bash ~/workspace/emacs-ai/var/package-profile-0.4.0/start.sh`.
새 독립 GUI를 여는 명령이며 개인 설정을 바꾸지 않는다. 기존 0.3.0 프로필은 유지했다.
GUI의 영역 선택·한글 의견 입력·키 동작은 아직 미검증이다.

## 0.8.7 GUI 직접 확인

Computer Use로 diff-mode의 변경 두 줄만 선택하고 C-c a v에서 영문 의견과 대상
세션을 입력해 초안을 확인했다. 이전/이후 example.py:1, 선택한 내용과 의견이 일치했고,
undo로 빈 초안 복원 및 C-g 취소 후 미변경을 확인했다. 앱 입력 저널은 0바이트다.
[실행 기록과 화면](evidence/gui-087/observations.md). 물리 한글 조합과 Magit GUI는
여전히 별도 확인 대상이다. 이전 단락의 GUI 미검증 상태는 이 후속 결과로 갱신한다.
