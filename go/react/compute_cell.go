package react

import "fmt"

type ComputeCellImpl struct {
	Cell
	//externalCallbacks []Callback
}

func (c *ComputeCellImpl) AddCallback(callback func(int)) Canceler {
	callbackCounter++
	callbackID := callbackCounter
	cell := getRootCell(c)
	fmt.Printf("[AddCallback] c(%v) cell(%v)\n", c, cell)

	cell.externalCallbacks = append(cell.externalCallbacks, Callback{
		id: callbackCounter,
		fn: callback,
	})
	//fmt.Printf("[AddCallback] result: %v\n", cell)

	fmt.Printf("[AddCallback] creating callback#%d\n", callbackID)
	return CancelerImpl{
		cancel: func() {
			var indexToRemove = -1
			for i, cb := range cell.externalCallbacks {
				if cb.id == callbackID {
					indexToRemove = i
					break
				}
			}
			if indexToRemove >= 0 {
				cell.externalCallbacks = append(
					cell.externalCallbacks[:indexToRemove],
					cell.externalCallbacks[indexToRemove+1:]...,
				)
			}
		},
	}
}

type CancelerImpl struct {
	cancel func()
}

func (c CancelerImpl) Cancel() {
	c.cancel()
}