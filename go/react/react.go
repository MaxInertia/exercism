package react

var callbackCounter = 0

type Callback struct {
	id int
	fn func(int)
}

func New() *ReactorImpl {
	return &ReactorImpl{cellCounter: 0}
}

type ReactorImpl struct{
	cellCounter int
}

func (r *ReactorImpl) CreateInput(value int) InputCell {
	return r.createInput(value)
}
func (r *ReactorImpl) createInput(value int) *InputCellImpl {
	r.cellCounter++
	return &InputCellImpl{
		id:                r.cellCounter,
		value:             value,
		callbacks:         make([]Callback, 0, 10),
		externalCallbacks: make([]Callback, 0, 10),
	}
}

func (r *ReactorImpl) CreateCompute1(cellIn Cell, fn func(int) int) ComputeCell {
	return r.createCompute1(cellIn, fn)
}
func (r *ReactorImpl) createCompute1(cellIn Cell, fn func(int) int) *ComputeCellImpl {
	cell := r.CreateInput(fn(cellIn.Value())).(*InputCellImpl)

	callbackCounter++
	addCallback(cellIn, Callback{
		id: callbackCounter,
		fn: func(v int) {
			cell.setValue(fn(v))
		},
	})

	return &ComputeCellImpl{Cell: cell}
}

func (r *ReactorImpl) CreateCompute2(cellIn Cell, cellIn2 Cell, fn func(int, int) int) ComputeCell {
	return r.createCompute2(cellIn, cellIn2, fn)
}
func (r *ReactorImpl) createCompute2(cellIn Cell, cellIn2 Cell, fn func(int, int) int) *ComputeCellImpl {
	newCell := r.CreateInput(fn(cellIn.Value(), cellIn2.Value())).(*InputCellImpl)

	callbackCounter++
	addCallback(cellIn, Callback{
		id: callbackCounter,
		fn: func(v int) {
			newCell.setValue(fn(v, cellIn2.Value()))
		},
	})

	callbackCounter++
	addCallback(cellIn2, Callback{
		id: callbackCounter,
		fn: func(v int) {
			newCell.setValue(fn(cellIn.Value(), v))
		},
	})

	return &ComputeCellImpl{Cell: newCell}
}

func addCallback(cell Cell, callback Callback) {
	//fmt.Printf("creating callback#%d\n", callback.id)
	root := getRootCell(cell)
	root.callbacks = append(root.callbacks, callback)
}

func getRootCell(cell Cell) *InputCellImpl {
	switch c := cell.(type) {
	case *InputCellImpl:
		return c
	case *ComputeCellImpl:
		return getRootCell(c.Cell)
	default:
		panic("unexpected cell type!")
	}
}
