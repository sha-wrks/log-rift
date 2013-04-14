import os

basedir = os.path.abspath(os.path.dirname(__file__))

DEBUG = True
SECRET_KEY = 'development-key-change-in-production'
SQLALCHEMY_DATABASE_URI = 'sqlite:///' + os.path.join(basedir, 'ordo.db')
SQLALCHEMY_TRACK_MODIFICATIONS = False
