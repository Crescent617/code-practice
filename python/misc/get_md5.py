#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import hashlib


def get_file_md5(file_name):
    m = hashlib.md5()  # 创建md5对象
    with open(file_name, 'rb') as f:
        for data in f:
            # print(len(data))
            m.update(data)  # 更新md5对象
    return m.hexdigest()  # 返回md5对象


if __name__ == '__main__':
    file_name = ""
    file_md5 = get_file_md5(file_name)
    print(file_name, file_md5)  # 0f45cdbf14de54001e82a17c3d199a4b
