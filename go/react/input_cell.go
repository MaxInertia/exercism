package react

import "fmt"

type InputCellImpl struct {
	id                int
	value             int
	callbacks         []Callback
	externalCallbacks []Callback
}

func (c *InputCellImpl) Value() int {
	return c.value
}

func (c *InputCellImpl) setValue(x int) {
	c.SetValue(x)
	//_setValue(c, x)
}

func _setValue(c *InputCellImpl, x int) {
	if c.value == x {
		return
	}

	c.value = x
	for _, callback := range c.callbacks {
		fmt.Printf("_calling callback#%d\n", callback.id)
		callback.fn(c.value)
	}
}

func (c *InputCellImpl) SetValue(x int) {
	if c.value == x {
		return
	}

	c.value = x
	if len(c.callbacks) > 0 {
		fmt.Print("calling callbacks: ")
		for _, callback := range c.callbacks {
			fmt.Printf("#%d ", callback.id)
			callback.fn(c.value)
		}
		fmt.Println()
	}

	if len(c.externalCallbacks) > 0 {
		fmt.Printf("calling EXTERNAL callbacks (%d):", len(c.externalCallbacks))
		for _, eCallback := range c.externalCallbacks {
			fmt.Printf("#%d ", eCallback.id)
			eCallback.fn(c.value)
		}
		fmt.Println()
	}
}