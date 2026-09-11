import os, sys, time
from pathlib import Path
name=sys.argv[1]
Path(name+'.pid').write_text(str(os.getpid()))
print('NORMAL_EXIT_PROBE_'+name, flush=True)
end=time.monotonic()+180
while time.monotonic()<end:
 Path(name+'.heartbeat').write_text(str(time.time()))
 time.sleep(.1)
