# Caffeine: 자동 잠자기 방지

2026-09-09 사용자 추가 요청 G13. 활성화 조건 답변이 없는 동안 기본 꺼짐·수동 켜기/끄기를 선택했다. 자동 활성화 선호가 도착하면 반영한다.

시작 화면의 **Caffeine 켜기/끄기** 또는 `M-x emacs-ai-caffeine-mode`를 사용한다. 켜지면 모드라인에 `Caffeine`이 표시된다. CLI가 열려 있다는 이유로 자동 활성화하지 않는다. 켜짐은 현재 Emacs 인스턴스에만 적용되며 개인 설정에 저장하지 않는다.

`/usr/bin/caffeinate -i -w EMACS_PID`로 시스템의 **유휴 자동 잠자기**를 막는다. 화면 잠자기를 막는 `-d`, 사용자 활동을 만드는 `-u`, 디스크 옵션은 사용하지 않는다. 뚜껑 닫기·강제 잠자기·배터리 소진 등 모든 중단을 막는 기능은 아니다. 설치된 macOS `man caffeinate`의 -i/-w 정의를 확인했다.

꺼짐은 앱이 만든 caffeinate 프로세스만 종료한다. Emacs 정상 종료 hook으로 정리하며 강제 종료 시에도 -w가 감시하던 Emacs 종료를 따라 assertion을 해제한다. 다른 앱의 caffeinate나 전원 설정을 건드리지 않는다. 별도 AI 호출·상태 조회 타이머는 없다.

## 검증

`python3 tests/caffeine-probe.py`로 실제 macOS assertion을 `pmset -g assertions`에서 앱 소유 PID별로 확인했다.

| 경우 | 결과 |
| --- | --- |
| 켜기·재차 켜기 | 같은 caffeinate 프로세스 유지, PreventUserIdleSystemSleep 생성 |
| 화면 잠자기 방지 | 해당 PID의 PreventUserIdleDisplaySleep assertion 없음 |
| 수동 끄기 | assertion 해제, 시험 Emacs는 살아 있음 |
| Emacs 정상 종료 | assertion 해제 |
| 시험 Emacs SIGKILL | assertion 해제 |

결과: `var/caffeine-probe-20260909.json`. 이 검사는 OS assertion 수명을 확인했으며 실제 Mac을 장시간 방치하거나 뚜껑을 닫아 시험하지 않았다. 후속 [일상 GUI 검사](daily-gui-validation.md)에서 실제 버튼의 켜기/끄기·모드라인 표시 추가/제거를 확인했다. 앱 회귀 4개도 통과해 시작 화면을 여는 것만으로 프로세스가 생성되지 않음을 확인했다.
