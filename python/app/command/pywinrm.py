import os
import winrm

winrm_host = os.environ.get('WINRM_HOST','http://192.168.1.1')
winrm_user = os.environ.get('WINRM_USER','')
winrm_password = os.environ.get('WINRM_PASSWORD','')




def getIpConfig():
  session = winrm.Session(winrm_host, auth=(winrm_user, winrm_password))
  r = session.run_cmd('ipconfig', ['/all'])

  print(r.status_code)
  print(r.std_out)
  print(r.std_err)
  print(type (r.std_out))

  # powershellで何も変えてなければsjisでデコードして返す
  return r.std_out.decode('sjis')

def getUsers():
  session = winrm.Session(winrm_host, auth=(winrm_user, winrm_password))
  r = session.run_ps('Get-ADUser -filter * | convertto-csv -Delimiter “,”')
  
  ## NOTE: run_cmdではなくrun_psを使う
  # r = session.run_cmd('Get-ADUser', ['-filter', '*'])

  print(r.status_code)
  print(r.std_out.decode('shift_jis'))
  print(r.std_err)
  print(type (r.std_out))
  
  # powershellで何も変えてなければsjisでデコードして返す
  return r.std_out.decode('shift_jis')