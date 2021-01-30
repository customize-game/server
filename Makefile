help:
	@echo "init    => Initialize enviroment settings."
	@echo "build    => Build enviroment settings."
	@echo "no-cache-build => Build enviroment settings. without cache"
	@echo "start    => Start Container."
	@echo "run    => Create Container."
	@echo "update  => Fetch all changes from remote repo."
	@echo "remove => Remove container."


build:
	@docker-compose build

start:
	@docker-compose start

run:
	@docker-compose up -d

stop:
	@docker-compose stop

exec:
	@docker exec -it customize_game_server bash

update:
	git pull origin master
	git submodule init
	git submodule update
	git submodule foreach git pull origin master

remove:
	@make stop
	@docker rm Customize_game_server_container
