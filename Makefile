# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: lumugot <lumugot@42angouleme.fr>           +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2026/04/10 16:30:36 by lumugot           #+#    #+#              #
#    Updated: 2026/04/21 00:10:36 by lumugot          ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = snake

all: build

check:
	@cargo check

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
	@rm -rf models

re : fclean all

.PHONY: all build run clean fclean purge re
