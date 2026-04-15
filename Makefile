# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2026/04/10 16:30:36 by lumugot           #+#    #+#              #
#    Updated: 2026/04/16 00:26:03 by lumugot          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = snake

all: build

build:
	@cargo build --release
	@cp target/release/$(NAME) .

run:
	@cargo run -- $(ARGS)

clean:
	@cargo clean -q

fclean: clean
	@rm -rf $(NAME)

purge:
	@rm -rf model.json 

re : fclean all

.PHONY: all build run clean fclean purge re
