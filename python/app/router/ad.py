from fastapi import APIRouter, Query, HTTPException
from command.pywinrm import getUsers
from fastapi.responses import PlainTextResponse

Router = APIRouter(
  prefix='/ad',
  tags=['ad']
)

@Router.get('/users', response_class=PlainTextResponse)
async def get_users():
  """
  ユーザー一覧を返す
  """
  return getUsers()