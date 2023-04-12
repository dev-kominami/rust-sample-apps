import os
import winrm

winrm_host = os.environ.get('WINRM_HOST','http://192.168.1.1')
winrm_user = os.environ.get('WINRM_USER','')
winrm_password = os.environ.get('WINRM_PASSWORD','')

session = winrm.Session(winrm_host, auth=(winrm_user, winrm_password))
r = session.run_cmd('ipconfig', ['/all'])

print(r.status_code)
print(r.std_out)
print(r.std_err)
  
  # return r.std_out