help:
	@echo "init    => Initialize enviroment settings."
	@echo "build    => Build enviroment settings."
	@echo "no-cache-build => Build enviroment settings. without cache"
	@echo "start    => Start Container."
	@echo "run    => Create Container."
	@echo "stop    => Stop Container."
	@echo "exec_rust    => execute To Rust Container."
	@echo "exec_mysql    => execute to  MySQL Container."
	@echo "update  => Fetch all changes from remote repo."
	@echo "remove => Remove Container."


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

exec_mysql:
	@docker exec -it customize_game_mysql bash

update:
	git pull origin master
	git submodule init
	git submodule update
	git submodule foreach git pull origin master

remove:
	@make stop
	@docker-compose down
