cmake_minimum_required(VERSION 3.14)

project(libhello)

add_library(hello STATIC
  hello.c
  proto.h)

  set_target_properties(hello PROPERTIES PUBLIC_HEADER "proto.h")

  install(TARGETS hello)