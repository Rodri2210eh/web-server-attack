from concurrent.futures import ThreadPoolExecutor
import sys


"""
    stress -n <cantidad-hilos> HTTPclient -h <host-a-conectar> [<lista-de-comandos-a-ejecutar>]
        1   2       3               4       5       6                       7
"""

def process_command(argumentos):
    len_args = len(argumentos)
    print("Numero de argumenos:" + str(len_args))
    print(argumentos)
    
    if len_args < 7:
        print("Error sintaxis: stress -n <cantidad-hilos> HTTPclient -h <host> [<lista-comandos-a-ejecutar>]")
        sys.exit()
    
    client = ['HTTPclient']

    for i in range(1, len_args):
        if (argumentos[i] == "-n"):
            threads = int(argumentos[i+1])
        elif (argumentos[i] == "-h"):
            client.extend(argumentos[i+1:])

    return [threads, client]



def main():
    argumentos = sys.argv
    (a, b) = process_command(argumentos)
    print(a)
    print(b)
    


main()

