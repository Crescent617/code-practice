import asyncio
import time


def now():
    return time.time()


async def tcp_echo_client(message):
    reader, writer = await asyncio.open_connection(
        '127.0.0.1', 8000)

    print(f'Send: {message!r}')
    writer.write(message.encode())

    data = await reader.read(100)
    print(f'Received: {data.decode()!r}')

    print('Close the connection')
    writer.close()


async def main(n):
    tasks = [tcp_echo_client('hello') for _ in range(n)]
    result = await asyncio.gather(*tasks)


if __name__ == "__main__":

    url = 'http://localhost:8888'

    start = now()

    # do_requests(100)
    asyncio.run(main(100))

    print('TIME: ', now() - start)
