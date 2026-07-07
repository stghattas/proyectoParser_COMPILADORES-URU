def configurar_entorno():
  x:int = 10
  y:int

def main():
      y:int = 5
      z:float = 14.5
      
      if y > 2:
        print("Entrando al primer nivel")
        
        if z < 20.0:
           resultado:float = (y * 10) + z / 2.0
           
           if resultado == 57.25:
              print("Calculo exacto logrado", resultado)
           else:
              print("Fallo de precision")
        else:
           print("Z es muy grande")