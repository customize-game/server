help:
	@echo "build    => Build enviroment settings."
	@echo "start    => Start Container."
	@echo "run    => Create Container."
	@echo "exec    => execute to rust Container."
	@echo "use_mysql    => execute MySQL Container."
	@echo "update  => Fetch all changes from remote repo."
	@echo "remove => Remove container."
	@echo "destroy => Remove garbage files"

build:
	@docker-compose build

start:
	@docker-compose start

run:
	@docker-compose up -d

stop:
	@docker-compose stop

exec_rust:
	@docker exec -it customize_game_server bash

use_mysql:
	@docker exec -it customize_game_mysql bash

update:
	git pull origin master
	git submodule init
	git submodule update
	git submodule foreach git pull origin master

remove:
	@make stop
	@docker rm customize_game_server
	@docker rm customize_game_mysql

destroy:
	@sudo rm -rf dockerfile/mysql
	@sudo rm -rf target/
