help:
	@echo "init    => Initialize enviroment settings."
	@echo "build    => Build enviroment settings."
	@echo "no-cache-build => Build enviroment settings. without cache"
	@echo "start    => Start Container."
	@echo "update  => Fetch all changes from remote repo."
	@echo "remove => Remove container."


build:
	@docker build . -f dockerfile/Dockerfile -t customize_game_server:latest

no-cache-build:
	@docker build . -f dockerfile/Dockerfile -t customize_game_server:latest --no-cache=true

start:
	@docker start Customize_game_server_container

run:
	@docker run -itd -v $$PWD/src:/opt/program -p 5000:5000 --name Customize_game_server_container customize_game_server:latest

stop:
	@docker stop Customize_game_server_container

exec:
	@docker exec -it Customize_game_server_container bash

update:
	git pull origin master
	git submodule init
	git submodule update
	git submodule foreach git pull origin master

remove:
	@make stop
	@docker rm Customize_game_server_container bash
